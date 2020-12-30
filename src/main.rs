use rss::Enclosure;
use rss::Guid;
use rss::Image;
use rss::Item;
use rss::extension::itunes::ITunesCategory;
use rss::extension::itunes::ITunesItemExtension;
use rss::extension::itunes::ITunesOwner;
use serde::{Deserialize, Serialize};
use toml;
use rss::{
    ChannelBuilder,
    Channel,
    ItemBuilder,
    extension::itunes::ITunesChannelExtension,
};

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

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
        println!("{}", data.to_string());
        println!("{}", format);
    }

}

// // NOTE: Returns map of "media format":"xml contents"
// fn generate_xmls(pod: &Podcast) -> Result<HashMap<String, String>, ()> {
//     let formats = pod.formats.clone();
//     let mut file_map = HashMap::new();

//     for format in formats {
//         let mut content_vec = vec![];
//         // TODO, generate!

//         // Podcast Body

//         // Fixed Header

//         content_vec.push(r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string());
//         content_vec.push(r#"<rss xmlns:atom="http://www.w3.org/2005/Atom" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" xmlns:itunesu="http://www.itunesu.com/feed" version="2.0">"#.to_string());
//         content_vec.push(r#"<channel>"#.to_string());

//         // Actual content

//         content_vec.push(format!(r#"<link>https://www.YourSite.com</link>"#));
//         content_vec.push(format!(r#"<language>en-us</language>"#));
//         content_vec.push(format!(r#"<copyright>&#xA9;2018</copyright>"#));
//         content_vec.push(format!(r#"<webMaster>your@email.com (Your Name)</webMaster>"#));
//         content_vec.push(format!(r#"<managingEditor>your@email.com (Your Name)</managingEditor>"#));
//         content_vec.push(format!(r#"<image>"#));
//         content_vec.push(format!(r#"   <url>https://www.YourSite.com/ImageSize300X300.jpg</url>"#));
//         content_vec.push(format!(r#"   <title>Title of your logo</title>"#));
//         content_vec.push(format!(r#"   <link>https://www.YourSite.com</link>"#));
//         content_vec.push(format!(r#"</image>"#));
//         content_vec.push(format!(r#"<itunes:owner>"#));
//         content_vec.push(format!(r#"   <itunes:name>Your Name</itunes:name>"#));
//         content_vec.push(format!(r#"   <itunes:email>your@email.com</itunes:email>"#));
//         content_vec.push(format!(r#"</itunes:owner>"#));
//         content_vec.push(format!(r#"<itunes:category text="Education">"#));
//         content_vec.push(format!(r#"   <itunes:category text="Higher Education" />"#));
//         content_vec.push(format!(r#"</itunes:category>"#));
//         content_vec.push(format!(r#"<itunes:keywords>separate, by, comma, and, space</itunes:keywords>"#));
//         content_vec.push(format!(r#"<itunes:explicit>no</itunes:explicit>"#));
//         content_vec.push(format!(r#"<itunes:image href="http://www.YourSite.com/ImageSize300X300.jpg" />"#));
//         content_vec.push(format!(r#"<atom:link href="https://www.YourSite.com/feed.xml" rel="self" type="application/rss+xml" />"#));
//         content_vec.push(format!(r#"<pubDate>Fri, 05 Oct 2018 09:00:00 GMT</pubDate>"#));
//         content_vec.push(format!(r#"<title>Verbose title of the podcast</title>"#));
//         content_vec.push(format!(r#"<itunes:author>College, school, or department owning the podcast</itunes:author>"#));
//         content_vec.push(format!(r#"<description>Verbose description of the podcast.</description>"#));
//         content_vec.push(format!(r#"<itunes:summary>Duplicate of above verbose description.</itunes:summary>"#));
//         content_vec.push(format!(r#"<itunes:subtitle>Short description of the podcast - 255 character max.</itunes:subtitle>"#));
//         content_vec.push(format!(r#"<lastBuildDate>Fri, 05 Oct 2018 09:00:00 GMT</lastBuildDate>"#));

//         // Episodes

//         // Podcast Footer
//         content_vec.push(r#"</channel>"#.to_string());
//         content_vec.push(r#"</rss>"#.to_string());

//         // END GENERATE
//         file_map.insert(format.to_string(), content_vec.join("\n"));
//     }

//     Ok(file_map)
// }

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
    itunes.set_explicit(
        Some(if pod.explicit {
            "Explicit"
        } else {
            "Clean"
        }.to_string())
    );

    itunes.set_owner(itunes_owner);
    itunes.set_subtitle(pod.subtitle);
    itunes.set_summary(pod.description.clone());
    itunes.set_keywords(pod.keywords.join(", "));

    // itunes.set_complete();
    // itunes.set_new_feed_url();
    // itunes.set_block();

    let mut namespaces: HashMap<String, String> = HashMap::new();

    namespaces.insert("atom".into(), "http://www.w3.org/2005/Atom".into());
    namespaces.insert("itunes".into(), "http://www.itunes.com/dtds/podcast-1.0.dtd".into());
    namespaces.insert("itunesu".into(), "http://www.itunesu.com/feed".into());

    let mut image = Image::default();
    image.set_url(pod.logo.url.clone());
    image.set_title(format!("{} Logo", &pod.title));
    image.set_link(pod.website.clone());

    // Generate everything EXCEPT the format
    let base_builder = cb.title(pod.title)
        .link(pod.website)
        .description(pod.description)
        .language(pod.language)
        .copyright(pod.copyright)
        .managing_editor(pod.managing_editor)
        .webmaster(pod.webmaster)
        .pub_date(Some("".into()))          // TODO! - This should be RIGHT NOW
        .last_build_date(Some("".into()))   // TODO! - This should be RIGHT NOW

        .generator(Some("pfg-rs".into()))   // TODO!

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
        itunes_item.set_duration(Some("00:00:00".into())); // lol
        itunes_item.set_explicit(Some(if pod.explicit {
            "Explicit"
        } else {
            "Clean"
        }.to_string()));
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
                    }.to_string();

                    encl.set_mime_type(mime);


                    let mut this_item = base_item.clone();
                    this_item.link(full_path.clone());
                    this_item.enclosure(encl);


                    this_item.guid(Some(guid));
                    cur_set.remove(ext);

                    item_map.entry(ext.to_string()).or_insert_with(|| vec![]).push(this_item.build().unwrap());
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
        this_builder.items(items);
        map.insert(ext.to_string(), this_builder.build().unwrap());
    }
    // TODO: Loop over items!
        // .items(todo!()) // TODO: This



    Ok(map)

}
