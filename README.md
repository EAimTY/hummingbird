# hummingbird

A lightweight and blazing fast content management system using git repo as the database.

## What exactly is hummingbird

hummingbird is a content management system uses a git repository as a database. It reads your posts and pages stored in the git repository in markdown format, applys HTML templates on them, then serves them.

hummingbird adheres **Less is better**. It has no complicated features or an omnipotent template system. What it can do is to present your content to everyone at the fastest speed dutifully.

hummingbird is written in Rust and built directly on top of [hyper](https://github.com/hyperium/hyper), which means fast and safe.

## Difference between hummingbird and traditional CMSs / GitHub Pages

### hummingbird vs traditional CMSs

- hummingbird runs tens times, if not hundreds times, faster than a traditional CMS
- Compares to setting up a traditional CMS, setting up hummingbird is extremely easy. No need to configure the runtime environment or DBMS. A configuration file and a git repository as database, those are what running hummingbird needs
- hummingbird's database is read-only - hummingbird fetches your repo when you call it. This makes hummingbird much safer than a traditional CMS
- hummingbird has much less features than a traditional CMS. You should only use hummingbird when you don't need a lot of features.
- hummingbird cannot handle frequent repo updates, such as once every minute. You should only use hummingbird when you are updating your content once every hour or less

### hummingbird vs GitHub Pages
- hummingbird is not a static HTML file serve-r. You can search keywords in your posts, even adding author and create-time filters on searches.
- hummingbird itself is a stand-alone binary file. You have to run it somewhere on a virtual server.

## Usage

1. [Build](#Build) or download hummingbird from [Releases](https://github.com/EAimTY/hummingbird/releases/latest)
2. Create a git repository on GitHub. Put your posts in `/posts/`, pages in `/pages/` and [HTML template](#Template) in `/template/`. Any other file in repo will be served as static.
3. Create a config file ([example.conf](https://github.com/EAimTY/hummingbird/blob/master/hummingbird.conf))
4. Run hummingbird with `/PATH/TO/HUMMINGBIRD -c CONFIG_FILE`
5. Access the update url set in the config to trigger a git repo fetch and a database update

hummingbird gets post and page infos like create time and author from the commit history of your database git repo. When you creating a post / page, you create a `.md` file in `/posts/` / `/pages/` in your repo and commit it. hummingbird reads the commit time and author as the post / page info.

You can use `<!--more-->` in your post. hummingbird only shows the content of the post above this `more` indicator when the post is showing in a list, like index or archive.

## Template

hummingbird has a simple yet adequate template framework:

- `/template/header.html`
- `/template/footer.html`
- `/template/page_nav.html`
- `/template/post.html`
- `/template/page.html`
- `/template/summary.html`
- `/template/not_found.html`

These are files hummingbird will read.

When getting a request, hummingbird will put them together in an order of:

`/template/header.html`

`/template/post.html` - if requesting a post

`/template/page.html` - if requesting a page

`/template/summary.html` - if requesting a list. hummingbird applies it on every posts in list and concats them

`/template/not_found.html` - if the requested page is not found or inaccessable

`/template/page_nav.html` - if requesting a list

`/template/footer.html`

You can use these patameters below in your template:


*Can be used in every template file:*

`{:site.url}` - Site URL set in config

`{:site.name}` - Site name set in config

`{:site.description}` - Site description set in config

`{:site.page_list}` - The list of all pages' title and link

`{:site.recent_posts}` - The list of recent posts' title and link

`{:document.title}` - The title of the current page, like the post name, archives time range, etc.

`{:document.url}` - The url of the current page

`{:document.breadcrumbs}` - The breadcrumbs, indicating current visiting location


*Can be used in `/template/page_nav.html`:*

`{:document.page_nav}` - The page navigator

`{:document.current_page_num_in_list}` - Current page number in the list

`{:document.total_num_of_articles_in_list}` - The total number of articles in the list


*Can be used in `/template/post.html`:*

`{:post.title}` - The title of the post

`{:post.link}` - The URL of the post

`{:post.content}` - The content of the post

`{:post.author}` - The author of the post

`{:post.create_time}` - The create time of the post

`{:post.modify_time}` - The last update time of the post


*Can be used in `/template/page.html`:*

`{:page.title}` - The title of the page

`{:page.link}` - The URL of the page

`{:page.content}` - The content of the page

`{:page.author}` - The author of the page

`{:page.create_time}` - The create time of the page

`{:page.modify_time}` - The last update time of the page


*Can be used in `/template/summary.html`:*

`{:summary.title}` - The title of the post in list

`{:summary.link}` - The URL of the post in list

`{:summary.summary}` - The summary of the post in list

`{:summary.author}` - The author of the post in list

`{:summary.create_time}` - The create time of the post in list

`{:summary.modify_time}` - The last update time of the post in list


To access a centain page number of a list, use the URL query `?page=PAGE_NUM`

To add a search filter, use URL queries like `?keyword=KEYWORD&time_range=START_TIMESTAMP-END_TIMESTAMP`

Search filters current supports:

- keyword: `keyword=KEYWORD`
- time range: `time_range=START_TIMESTAMP-END_TIMESTAMP`
- author: `author=AUTHOR`

## Build

Rust 1.56 or above is required to compile hummingbird.

```bash
$ git clone https://github.com/EAimTY/hummingbird && cd hummingbird
$ cargo build --release
```

## Further development plan

- Built-in TLS support
- Multiple income socket support
- Full evaluate-on-write template-applying
- Read post / page properties from file content
- Rewrite the route table structure
- Longpoll database updating
- More template parameters
- More search filters
- ...

## License

GNU General Public License v3.0
