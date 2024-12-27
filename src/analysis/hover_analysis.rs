use std::collections::HashMap;
use crate::lsp::lsp_struct::Position;

pub fn get_hover_data(content:&str, position:&Position)->String {

  let desc_list = get_desc_list();
  let word = get_word(content, position);
  let empty_val = String::from("");
  let desc = desc_list.get(&word).unwrap_or(&empty_val);
  return desc.to_string();
}

fn get_word(content:&str, position:&Position)->String{
  let seperators = ['<', '>', '/'];
  let mut is_starting = false;
  if let Some(line) = content.lines().nth(position.line as usize) {
    let mut word = String::from(""); 
    for (index, char) in line.chars().skip(position.character as usize).enumerate() {
      if index==0 && char=='<' {
        is_starting = true;
        continue;
      } else if char=='/' {
        continue;
      } else if seperators.contains(&char) {
        break;
      }
      if char==' ' {
        break;
      }
      word.push(char);
    }
    if is_starting {
      return word;
    }
    for char in line.chars().rev().skip(line.len() - position.character as usize) {
      if seperators.contains(&char) {
        break;
      }
      if char==' ' {
        return "".to_string();
      }
      word= format!("{}{}", char, word);
    }

    return word;

  } else {
    return String::from("");
  }
}
fn get_desc_list()->HashMap<String, String> {
  let mut desc_list:HashMap<String, String> = HashMap::new();
  desc_list.insert("html".to_string(), "The html element represents the root of an HTML document.".to_string());
  desc_list.insert("head".to_string(), "The head element represents a collection of metadata for the Document.".to_string());
  desc_list.insert("title".to_string(), "The title element represents the document's title or name. Authors should use titles that identify their documents even when they are used out of context, for example in a user's history or bookmarks, or in search results. The document's title is often different from its first heading, since the first heading does not have to stand alone when taken out of context.".to_string());
  desc_list.insert("meta".to_string(), "The meta element represents various kinds of metadata that cannot be expressed using the title, base, link, style, and script elements.".to_string());
  desc_list.insert("link".to_string(), "The link element allows authors to link their document to other resources".to_string());
  desc_list.insert("style".to_string(), "The style element allows authors to embed style information in their documents. The style element is one of several inputs to the styling processing model. The element does not represent content for the user.".to_string());
  desc_list.insert("body".to_string(), "The body element represents the content of the document.".to_string());
  desc_list.insert("h1".to_string(), "The h1 element represents a section heading.".to_string());
  desc_list.insert("h2".to_string(), "The h2 element represents a section heading.".to_string());
  desc_list.insert("h3".to_string(), "The h3 element represents a section heading.".to_string());
  desc_list.insert("h4".to_string(), "The h4 element represents a section heading.".to_string());
  desc_list.insert("h5".to_string(), "The h5 element represents a section heading.".to_string());
  desc_list.insert("h6".to_string(), "The h6 element represents a section heading.".to_string());
  desc_list.insert("p".to_string(), "The p element represents a paragraph.".to_string());
  desc_list.insert("main".to_string(), "The main element represents the main content of the body of a document or application. The main content area consists of content that is directly related to or expands upon the central topic of a document or central functionality of an application.".to_string());
  desc_list.insert("section".to_string(), "The section element represents a generic section of a document or application. A section, in this context, is a thematic grouping of content. Each section should be identified, typically by including a heading ( h1- h6 element) as a child of the section element.".to_string());
  desc_list.insert("article".to_string(), "The article element represents a complete, or self-contained, composition in a document, page, application, or site and that is, in principle, independently distributable or reusable, e.g. in syndication. This could be a forum post, a magazine or newspaper article, a blog entry, a user-submitted comment, an interactive widget or gadget, or any other independent item of content. Each article should be identified, typically by including a heading (h1–h6 element) as a child of the article element.".to_string());
  desc_list.insert("nav".to_string(), "The nav element represents a section of a page that links to other pages or to parts within the page: a section with navigation links.".to_string());
  desc_list.insert("aside".to_string(), "The aside element represents a section of a page that consists of content that is tangentially related to the content around the aside element, and which could be considered separate from that content. Such sections are often represented as sidebars in printed typography.".to_string());
  desc_list.insert("hgroup".to_string(), "The hgroup element represents a heading and related content. It groups a single h1–h6 element with one or more p.".to_string());
  desc_list.insert("blockquote".to_string(), "The blockquote element represents content that is quoted from another source, optionally with a citation which must be within a footer or cite element, and optionally with in-line changes such as annotations and abbreviations.".to_string());
  desc_list.insert("base".to_string(), "The base element allows authors to specify the document base URL for the purposes of resolving relative URLs, and the name of the default browsing context for the purposes of following hyperlinks. The element does not represent any content beyond this information.".to_string());
  desc_list.insert("div".to_string(), "The div element has no special meaning at all. It represents its children. It can be used with the class, lang, and title attributes to mark up semantics common to a group of consecutive elements.".to_string());
  desc_list.insert("span".to_string(), "The span element doesn't mean anything on its own, but can be useful when used together with the global attributes, e.g. class, lang, or dir. It represents its children.".to_string());
  desc_list.insert("pre".to_string(), "The pre element represents a block of preformatted text, in which structure is represented by typographic conventions rather than by elements.".to_string());
  desc_list.insert("code".to_string(), "The code element represents a fragment of computer code. This could be an XML element name, a file name, a computer program, or any other string that a computer would recognize.".to_string());
  desc_list.insert("kbd".to_string(), "The kbd element represents user input (typically keyboard input, although it may also be used to represent other input, such as voice commands).".to_string());
  desc_list.insert("var".to_string(), "The var element represents a variable. This could be an actual variable in a mathematical expression or programming context, an identifier representing a constant, a symbol identifying a physical quantity, a function parameter, or just be a term used as a placeholder in prose.".to_string());
  desc_list.insert("samp".to_string(), "The samp element represents sample or quoted output from another program or computing system.".to_string());
  desc_list.insert("abbr".to_string(), "The abbr element represents an abbreviation or acronym, optionally with its expansion. The title attribute may be used to provide an expansion of the abbreviation. The attribute, if specified, must contain an expansion of the abbreviation, and nothing else.".to_string());
  desc_list.insert("city".to_string(), "The cite element represents a reference to a creative work. It must include the title of the work or the name of the author(person, people or organization) or an URL reference, or a reference in abbreviated form as per the conventions used for the addition of citation metadata.".to_string());
  desc_list.insert("dfn".to_string(), "The dfn element represents the defining instance of a term. The paragraph, description list group, or section that is the nearest ancestor of the dfn element must also contain the definition(s) for the term given by the dfn element.".to_string());
  desc_list.insert("time".to_string(), "The time element represents its contents, along with a machine-readable form of those contents in the datetime attribute. The kind of content is limited to various kinds of dates, times, time-zone offsets, and durations, as described below.".to_string());
  desc_list.insert("mark".to_string(), "The mark element represents a run of text in one document marked or highlighted for reference purposes, due to its relevance in another context. When used in a quotation or other block of text referred to from the prose, it indicates a highlight that was not originally present but which has been added to bring the reader's attention to a part of the text that might not have been considered important by the original author when the block was originally written, but which is now under previously unexpected scrutiny. When used in the main prose of a document, it indicates a part of the document that has been highlighted due to its likely relevance to the user's current activity.".to_string());
  desc_list.insert("b".to_string(), "The b element represents a span of text to which attention is being drawn for utilitarian purposes without conveying any extra importance and with no implication of an alternate voice or mood, such as key words in a document abstract, product names in a review, actionable words in interactive text-driven software, or an article lede.".to_string());
  desc_list.insert("strong".to_string(), "The strong element represents strong importance, seriousness, or urgency for its contents.".to_string());
  desc_list.insert("i".to_string(), "The i element represents a span of text in an alternate voice or mood, or otherwise offset from the normal prose in a manner indicating a different quality of text, such as a taxonomic designation, a technical term, an idiomatic phrase from another language, transliteration, a thought, or a ship name in Western texts.".to_string());
  desc_list.insert("em".to_string(), "The em element represents stress emphasis of its contents.".to_string());
  desc_list.insert("samll".to_string(), "The small element represents side comments such as small print.".to_string());
  desc_list.insert("u".to_string(), "The u element represents a span of text with an unarticulated, though explicitly rendered, non-textual annotation, such as labeling the text as being a proper name in Chinese text (a Chinese proper name mark), or labeling the text as being misspelt.".to_string());
  desc_list.insert("del".to_string(), "The del element represents a removal from the document.".to_string());
  desc_list.insert("ins".to_string(), "The ins element represents an addition to the document.".to_string());
  desc_list.insert("br".to_string(), "The br element represents a line break.".to_string());
  desc_list.insert("wbr".to_string(), "The wbr element represents a line break opportunity.".to_string());
  desc_list.insert("ul".to_string(), "The ul element represents a list of items, where the order of the items is not important — that is, where changing the order would not materially change the meaning of the document.".to_string());
  desc_list.insert("ol".to_string(), "The ol element represents a list of items, where the items have been intentionally ordered, such that changing the order would change the meaning of the document.".to_string());
  desc_list.insert("li".to_string(), "The li element represents a list item. If its parent element is an ol, ul, or menu element, then the element is an item of the parent element's list, as defined for those elements. Otherwise, the list item has no defined list-related relationship to any other li element.".to_string());
  desc_list.insert("dl".to_string(), "The dl element represents an association list consisting of zero or more name-value groups (a description list). A name-value group consists of one or more names (dt elements) followed by one or more values (dd elements), ignoring any nodes other than dt and dd elements. Within a single dl element, there should not be more than one dt element for each name.".to_string());
  desc_list.insert("dt".to_string(), "The dt element represents the term, or name, part of a term-description group in a description list (dl element).".to_string());
  desc_list.insert("dd".to_string(), "The dd element represents the description, definition, or value, part of a term-description group in a description list (dl element).".to_string());
  desc_list.insert("a".to_string(), "If the a element has an href attribute, then it represents a hyperlink (a hypertext anchor) labeled by its contents.".to_string());
  desc_list.insert("area".to_string(), "The area element represents either a hyperlink with some text and a corresponding area on an image map, or a dead area on an image map.".to_string());
  desc_list.insert("img".to_string(), "An img element represents an image.".to_string());
  desc_list.insert("figure".to_string(), "The figure element represents some flow content, optionally with a caption, that is self-contained (like a complete sentence) and is typically referenced as a single unit from the main flow of the document.".to_string());
  desc_list.insert("figcaption".to_string(), "The figcaption element represents a caption or legend for the rest of the contents of the figcaption element's parent figure element, if any.".to_string());
  desc_list.insert("picture".to_string(), "The picture element is a container which provides multiple sources to its contained img element to allow authors to declaratively control or give hints to the user agent about which image resource to use, based on the screen pixel density, viewport size, image format, and other factors. It represents its children.".to_string());
  desc_list.insert("audio".to_string(), "An audio element represents a sound or audio stream.".to_string());
  desc_list.insert("video".to_string(), "A video element is used for playing videos or movies, and audio files with captions.".to_string());
  desc_list.insert("source".to_string(), "The source element allows authors to specify multiple alternative media resources for media elements. It does not represent anything on its own.".to_string());
  desc_list.insert("track".to_string(), "The track element allows authors to specify explicit external timed text tracks for media elements. It does not represent anything on its own.".to_string());
  desc_list.insert("iframe".to_string(), "The iframe element represents a nested browsing context.".to_string());
  desc_list.insert("embed".to_string(), "The embed element provides an integration point for an external (typically non-HTML) application or interactive content.".to_string());
  desc_list.insert("object".to_string(), "The object element can represent an external resource, which, depending on the type of the resource, will either be treated as an image, as a nested browsing context, or as an external resource to be processed by a plugin.".to_string());
  desc_list.insert("param".to_string(), "The param element defines parameters for plugins invoked by object elements. It does not represent anything on its own".to_string());
  desc_list.insert("table".to_string(), "The table element represents data with more than one dimension, in the form of a table.".to_string());
  desc_list.insert("caption".to_string(), "The caption element represents the title of the table that is its parent, if it has a parent and that is a table element.".to_string());
  desc_list.insert("thead".to_string(), "The thead element represents the block of rows that consist of the column labels (headers) for the parent table element, if the thead element has a parent and it is a table.".to_string());
  desc_list.insert("tbody".to_string(), "The tbody element represents a block of rows that consist of a body of data for the parent table element, if the tbody element has a parent and it is a table.".to_string());
  desc_list.insert("tr".to_string(), "The tr element represents a row of cells in a table.".to_string());
  desc_list.insert("th".to_string(), "The th element represents a header cell in a table.".to_string());
  desc_list.insert("td".to_string(), "The td element represents a data cell in a table.".to_string());
  desc_list.insert("col".to_string(), "If a col element has a parent and that is a colgroup element that itself has a parent that is a table element, then the col element represents one or more columns in the column group represented by that colgroup.".to_string());
  desc_list.insert("colgroup".to_string(), "The colgroup element represents a group of one or more columns in the table that is its parent, if it has a parent and that is a table element.".to_string());
  desc_list.insert("form".to_string(), "The form element represents a collection of form-associated elements, some of which can represent editable values that can be submitted to a server for processing.".to_string());
  desc_list.insert("input".to_string(), "The input element represents a typed data field, usually with a form control to allow the user to edit the data.".to_string());
  desc_list.insert("textarea".to_string(), "The textarea element represents a multiline plain text edit control for the element's raw value. The contents of the control represent the control's default value.".to_string());
  desc_list.insert("button".to_string(), "The button element represents a button labeled by its contents.".to_string());
  desc_list.insert("select".to_string(), "The select element represents a control for selecting amongst a set of options.".to_string());
  desc_list.insert("option".to_string(), "The option element represents an option in a select element or as part of a list of suggestions in a datalist element.".to_string());
  desc_list.insert("optgroup".to_string(), "The optgroup element represents a group of option elements with a common label.".to_string());
  desc_list.insert("label".to_string(), "The label element represents a caption in a user interface. The caption can be associated with a specific form control, known as the label element's labeled control, either using the for attribute, or by putting the form control inside the label element itself.".to_string());
  desc_list.insert("datalist".to_string(), "The datalist element represents a set of option elements that represent predefined options for other controls. In the rendering, the datalist element represents nothing and it, along with its children, should be hidden.".to_string());
  desc_list.insert("output".to_string(), "The output element represents the result of a calculation performed by the application, or the result of a user action.".to_string());
  desc_list.insert("progress".to_string(), "The progress element represents the completion progress of a task. The progress is either indeterminate, indicating that progress is being made but that it is not clear how much more work remains to be done before the task is complete (e.g. because the task is waiting for a remote host to respond), or the progress is a number in the range zero to a maximum, giving the fraction of work that has so far been completed.".to_string());
  desc_list.insert("meter".to_string(), "The meter element represents a scalar measurement within a known range, or a fractional value; for example disk usage, the relevance of a query result, or the fraction of a voting population to have selected a particular candidate.".to_string());
  desc_list.insert("canvas".to_string(), "The canvas element provides scripts with a resolution-dependent bitmap canvas, which can be used for rendering graphs, game graphics, art, or other visual images on the fly.".to_string());
  desc_list.insert("summary".to_string(), "The summary element represents a summary, caption, or legend for the rest of the contents of the summary element's parent details element, if any.".to_string());
  desc_list.insert("details".to_string(), "The details element represents a disclosure widget from which the user can obtain additional information or controls.".to_string());
  desc_list.insert("dialog".to_string(), "The dialog element represents a part of an application that a user interacts with to perform a task, for example a dialog box, inspector, or window.".to_string());
  desc_list.insert("template".to_string(), "The template element is used to declare fragments of HTML that can be cloned and inserted in the document by script.".to_string());
  desc_list.insert("slot".to_string(), "The slot element is a placeholder inside a web component that you can fill with your own markup, which lets you create separate DOM trees and present them together.".to_string());
  desc_list.insert("header".to_string(), "The header element represents introductory content for its nearest ancestor sectioning content or sectioning root element. A header typically contains a group of introductory or navigational aids. When the nearest ancestor sectioning content or sectioning root element is the body element, then it applies to the whole page.".to_string());
  desc_list.insert("footer".to_string(), "The footer element represents a footer for its nearest ancestor sectioning content or sectioning root element. A footer typically contains information about its section such as who wrote it, links to related documents, copyright data, and the like.".to_string());
  desc_list.insert("script".to_string(), "The script element allows authors to include dynamic script and data blocks in their documents. The element does not represent content for the user.".to_string());
  desc_list.insert("noscript".to_string(), "The noscript element represents nothing if scripting is enabled, and represents its children if scripting is disabled. It is used to present different markup to user agents that support scripting and those that don't support scripting, by affecting how the document is parsed.".to_string());
  return desc_list;
}
