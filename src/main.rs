use rss::{
    extension::itunes::{ITunesCategory, ITunesChannelExtension, ITunesItemExtension, ITunesOwner},
    Channel, ChannelBuilder, Enclosure, Guid, Image, Item, ItemBuilder,
};
use serde::{Deserialize, Serialize};

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::prelude::*,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct Podcast {
    title: String,
    description: String,
    subtitle: String,
    author: String,
    author_email: String,
    website: String,
    language: String,
    copyright: String,
    webmaster: String,
    managing_editor: String, // TODO: kebab-case
    formats: Vec<String>,
    hosting_base_url: String, // TODO: kebab-case

    keywords: Vec<String>,
    explicit: bool,

    // TODO: Do we even need separate Logo data?
    logo: Logo,
    category: String,

    episodes: Vec<Episode>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct Logo {
    url: String,
    title: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct ItunesOwner {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct ItunesCategory {
    // ?
    text: String,
    itunesu_category: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct Episode {
    title: String,
    url: String,
    description: String,
    subtitle: String,
    files: Vec<String>,
    duration: String,     // TODO: NaiveTime?
    publish_date: String, // TODO: NaiveDateTime/DateTime, kebab-case?
    keywords: Vec<String>,
}

fn main() {
    let mut buffer = String::new();
    let mut in_file = File::open("example.toml").unwrap();
    in_file.read_to_string(&mut buffer).unwrap();
    let podcast = toml::from_str::<Podcast>(&buffer).unwrap();

    // println!("{:#?}", podcast);

    let xmls = generate_xmls(podcast).unwrap();

    for (format, data) in xmls.iter() {
        let filename = format!("podcast-feed-{}.xml", format);
        println!("Writing '{}'...", &filename);

        let mut file = File::create(&filename).unwrap();
        let unformatted = data.to_string();
        let formatted = format_xml(unformatted.as_bytes()).unwrap();
        file.write_all(formatted.as_bytes()).unwrap();
    }
}

fn generate_xmls(pod: Podcast) -> Result<HashMap<String, Channel>, ()> {
    let mut cb = ChannelBuilder::default();

    let mut itunes = ITunesChannelExtension::default();

    let mut itunes_category = ITunesCategory::default();
    itunes_category.set_text(pod.category);

    let mut itunes_owner = ITunesOwner::default();
    itunes_owner.set_name(Some(pod.author.clone()));
    itunes_owner.set_email(Some(pod.author_email.clone()));

    itunes.set_author(pod.author.clone());
    itunes.set_categories(vec![itunes_category]);
    itunes.set_image(pod.logo.url.clone());
    itunes.set_explicit(Some(
        if pod.explicit { "Explicit" } else { "Clean" }.to_string(),
    ));

    itunes.set_owner(itunes_owner);
    itunes.set_subtitle(pod.subtitle);
    itunes.set_summary(pod.description.clone());
    itunes.set_keywords(pod.keywords.join(", "));

    // itunes.set_complete();
    // itunes.set_new_feed_url();
    // itunes.set_block();

    let mut namespaces: BTreeMap<String, String> = BTreeMap::new();

    namespaces.insert("atom".into(), "http://www.w3.org/2005/Atom".into());
    namespaces.insert(
        "itunes".into(),
        "http://www.itunes.com/dtds/podcast-1.0.dtd".into(),
    );
    namespaces.insert("itunesu".into(), "http://www.itunesu.com/feed".into());

    let mut image = Image::default();
    image.set_url(pod.logo.url.clone());
    image.set_title(format!("{} Logo", &pod.title));
    image.set_link(pod.website.clone());

    // Generate everything EXCEPT the format
    let base_builder = cb
        .title(pod.title)
        .link(pod.website)
        .description(pod.description)
        .language(pod.language)
        .copyright(pod.copyright)
        .managing_editor(pod.managing_editor)
        .webmaster(pod.webmaster)
        .pub_date(Some("".into())) // TODO! - This should be RIGHT NOW
        .last_build_date(Some("".into())) // TODO! - This should be RIGHT NOW
        .generator(Some("pfg-rs".into())) // TODO!
        .image(image)
        .itunes_ext(itunes)
        .namespaces(namespaces);

    // Currently unused items
    //
    // .extensions(todo!()) // TODO: This, maybe? Anything other than itunes?
    // .text_input(todo!())
    // .skip_hours(todo!())
    // .skip_days(todo!())
    // .categories(todo!())
    // .docs(todo!())
    // .cloud(todo!())
    // .rating(todo!())
    // .ttl(todo!())
    // .dublin_core_ext(todo!())
    // .syndication_ext(todo!())

    let mut map = HashMap::new();

    let mut item_map: HashMap<String, Vec<Item>> = HashMap::new();

    let base_set: HashSet<_> = pod.formats.clone().drain(..).collect();

    for episode in pod.episodes {
        let mut itunes_item = ITunesItemExtension::default();
        itunes_item.set_author(Some(pod.author.clone()));
        // itunes_item.set_block();
        itunes_item.set_image(Some(pod.logo.url.clone()));
        itunes_item.set_duration(Some(episode.duration)); // lol
        itunes_item.set_explicit(Some(
            if pod.explicit { "Explicit" } else { "Clean" }.to_string(),
        ));
        itunes_item.set_summary(episode.description.clone());
        itunes_item.set_subtitle(episode.subtitle.clone());
        itunes_item.set_keywords(episode.keywords.join(", "));

        // Make "base" builder
        let mut base_item = ItemBuilder::default();

        base_item
            .title(episode.title.clone())
            .description(episode.description.clone())
            .author(pod.author_email.clone()) // email
            // .categories()
            // .comments() // URL

            .pub_date(episode.publish_date.clone()) // RFC822
            // .source() // This RSS feed?
            // .content() // ?
            .itunes_ext(itunes_item) // TODO
            // .dublin_core_ext()
            ;

        // .link() // Do in format
        // .enclosure() // Do in format?
        // .guid() // Do in format?

        let mut cur_set = base_set.clone();

        for file in episode.files {
            let ext = file.split('.').last().unwrap();
            let full_path = format!("{}/{}", pod.hosting_base_url, file);

            match (base_set.contains(ext), cur_set.contains(ext)) {
                (true, true) => {
                    let mut guid = Guid::default();
                    guid.set_value(full_path.clone());
                    guid.set_permalink(true);

                    let mut encl = Enclosure::default();
                    encl.set_url(full_path.clone());

                    let mime = match ext.to_lowercase().as_str() {
                        "mp3" => "audio/mpeg",
                        "m4a" => "audio/mp4",
                        "flac" => "audio/flac",
                        _ => "",
                    }
                    .to_string();

                    encl.set_mime_type(mime);

                    let mut this_item = base_item.clone();
                    this_item.link(episode.url.clone());
                    this_item.enclosure(encl);

                    this_item.guid(Some(guid));
                    cur_set.remove(ext);

                    item_map
                        .entry(ext.to_string())
                        .or_default()
                        .push(this_item.build());
                }
                (true, false) => {
                    eprintln!("We've already added a file of format '{}' for episode '{}'. Skipping file '{}' with duplicate format.", ext, episode.title, file)
                }
                (false, _) => {
                    eprintln!("This podcast does not have '{}' in the listed 'formats'! Skipping '{}' in episode '{}'.", ext, file, episode.title);
                }
            }
        }
    }

    for (ext, items) in item_map.drain() {
        let mut this_builder = base_builder.clone();
        println!("{:#?}", items);
        this_builder.items(items);
        map.insert(ext.to_string(), this_builder.build());
    }

    Ok(map)
}

use xml::{reader::ParserConfig, writer::EmitterConfig};

// https://users.rust-lang.org/t/pretty-printing-xml/76372/3
fn format_xml(src: &[u8]) -> Result<String, xml::reader::Error> {
    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(src);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .normalize_empty_elements(false)
        .autopad_comments(false)
        .create_writer(&mut dest);
    for event in reader {
        if let Some(event) = event?.as_writer_event() {
            writer.write(event).unwrap();
        }
    }
    Ok(String::from_utf8(dest).unwrap())
}
