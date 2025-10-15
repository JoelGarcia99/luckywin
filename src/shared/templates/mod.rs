use std::fmt::Write;

const TEMPLATE: &str = include_str!("luckywin_ticket.html");

#[derive(Debug, Clone)]
pub struct LuckyNumberSet {
    pub title: String,
    pub digits: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LuckywinTicketContext {
    pub client_name: String,
    pub client_cellphone: String,
    pub game_name: String,
    pub game_serial: String,
    pub purchase_serial: String,
    pub purchase_date: String,
    pub game_closes_at: String,
    pub total_paid: String,
    pub number_sets: Vec<LuckyNumberSet>,
}

fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn build_number_sets_html(number_sets: &[LuckyNumberSet]) -> String {
    let mut html = String::new();

    for set in number_sets {
        let _ = write!(
            html,
            r#"<div class="number-set"><span class="set-label">{}</span><div class="digits">"#,
            escape_html(&set.title)
        );

        for digit in &set.digits {
            let _ = write!(html, r#"<span class="digit">{}</span>"#, escape_html(digit));
        }

        html.push_str("</div></div>");
    }

    html
}

pub fn render_luckywin_ticket(context: &LuckywinTicketContext) -> String {
    let mut html = TEMPLATE.to_string();

    let replacements = [
        ("{{CLIENT_NAME}}", escape_html(&context.client_name)),
        (
            "{{CLIENT_CELLPHONE}}",
            escape_html(&context.client_cellphone),
        ),
        ("{{GAME_NAME}}", escape_html(&context.game_name)),
        ("{{GAME_SERIAL}}", escape_html(&context.game_serial)),
        ("{{PURCHASE_SERIAL}}", escape_html(&context.purchase_serial)),
        ("{{PURCHASE_DATE}}", escape_html(&context.purchase_date)),
        ("{{GAME_CLOSES_AT}}", escape_html(&context.game_closes_at)),
        ("{{TOTAL_PAID}}", escape_html(&context.total_paid)),
    ];

    for (placeholder, value) in replacements {
        html = html.replace(placeholder, &value);
    }

    let numbers_html = build_number_sets_html(&context.number_sets);
    html = html.replace("{{NUMBER_SETS}}", &numbers_html);

    html
}
