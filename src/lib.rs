use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized
}

#[derive(PartialEq, Debug)]
pub enum TagType {
    ForTag,
    IfTag
}

#[derive(PartialEq, Debug)]
pub struct  ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

/// Accepts an input statement and tokenizes it into one of an if tag, a for tag, or a template variable.
pub fn get_content_type(input_line: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(
        &input_line, "{%", "%}");
        
    let is_for_tag = check_symbol_string(
        &input_line, "for") 
        && check_symbol_string(&input_line, "in")
        || check_symbol_string(&input_line, "endfor");

    let is_if_tag = check_symbol_string(&input_line, "if") 
        || check_symbol_string(&input_line, "endif");
    
    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");
    let return_val;

    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag)
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    } else {
        return_val = ContentType::Unrecognized;
    }
    
    return_val
}

/// Checks if a symbol is present within another string. 
/// 
/// For example, we can check if the pattern {% is present 
/// in a statement from the template file, and use it 
/// to determine if it is a tag statement or template variable.
pub fn check_symbol_string(input_line: &str, symbol: &str) -> bool {
    input_line.contains(symbol)
}

/// Used to verify if a statement in a template file is syntactically correct. 
/// 
/// For example, we can check for the presence of matching pairs {% and %}. 
/// Otherwise, the statement is marked as Unrecognized.
pub fn check_matching_pair(input_line: &str, left: &str, right: &str) -> bool {
    input_line.contains(left) && input_line.contains(right)
}

/// This method returns the starting index of a substring within another string. 
pub fn get_index_for_symbol(input_line: &str, symbol: char) -> (bool, usize) {
    let mut characters = input_line.char_indices();
    let mut exist = false;
    let mut index = 0;
    while let Some((c, d)) = characters.next() {
        if d == symbol {
            exist = true;
            index = c;
            break;
        }
    }

    (exist, index)
}

/// This method parses a template string into its constituent parts for a token of type TemplateString.
pub fn get_expression_data(input_line: &str) -> ExpressionData {
    let (_h, i) = get_index_for_symbol(input_line, '{');
    let head = input_line[0..i].to_string();

    let (_j, k) = get_index_for_symbol(input_line, '}');
    let variable = input_line[i + 1 + 1..k].to_string();

    let tail = input_line[k + 1 + 1..].to_string();

    ExpressionData { 
        head: Some(head), 
        variable, 
        tail: Some(tail) 
    }
}

pub fn generate_html_template_var(content: ExpressionData, context: HashMap<String, String>) -> String {
    let mut html = String::new();

    if let Some(h) = content.head {
        html.push_str(&h);
    }

    if let Some(val) = context.get(&content.variable) {
        html.push_str(val);
    }

    if let Some(t) = content.tail {
        html.push_str(&t);
    }

    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s = "<h1>Hello world</h1>";
        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    #[test]
    fn check_template_var_test() {
        let content = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" ,welcome".to_string()),
        };
        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}} ,welcome")
        );
    }

    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %} ,welcome")
        );
    }

    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' %} ,welcome")
        );
    }

    #[test]
    fn check_symbol_string_test() {
        assert_eq!(
            true, check_symbol_string("{{Hello}}", "{{")
        );
    }

    #[test]
    fn check_symbol_pair_test() {
        assert_eq!(
            true, check_matching_pair("{{Hello}}", "{{", "}}")
        );
    }

    #[test]
    fn check_get_expression_data_test() {
        let expression_data = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" ,welcome".to_string()),
        };

        assert_eq!(
            expression_data, get_expression_data("Hi {{name}} ,welcome")
        );
    }

    #[test]
    fn check_get_index_for_symbol_test() {
        assert_eq!(
            (true, 3), get_index_for_symbol("Hi {name}, welcome", '{')
        );
    }
}