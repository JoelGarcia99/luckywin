use std::fmt::Write;

const TEMPLATE: &str = include_str!("luckywin_ticket.html");

pub mod assets;

#[derive(Debug, Clone)]
pub struct LuckyNumberSet {
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
    pub environment: String,
    pub is_paid: bool,
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
        html.push_str(r#"<div class="number-set">"#);

        for digit in &set.digits {
            let _ = write!(html, r#"<div class="digit">{}</div>"#, escape_html(digit));
        }

        html.push_str("</div>");
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

    // Add watermark if not production
    let watermark_html = if context.environment.to_lowercase() != "prod" {
        r#"<div class="watermark">PRUEBAS</div>"#
    } else {
        ""
    };
    html = html.replace("{{WATERMARK}}", watermark_html);

    // Add paid signature if purchase is paid
    let paid_signature = if context.is_paid {
        r###"<div class="paid-signature">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" role="img" aria-labelledby="stamp-title">
                <title id="stamp-title">Sello PAGADO</title>
                <defs>
                    <linearGradient id="gold" x1="0%" y1="0%" x2="100%" y2="100%">
                        <stop offset="0%" stop-color="#b8860b"/>
                        <stop offset="45%" stop-color="#d4af37"/>
                        <stop offset="100%" stop-color="#f2d17b"/>
                    </linearGradient>
                    <filter id="softShadow" x="-20%" y="-20%" width="140%" height="140%">
                        <feDropShadow dx="0" dy="4" stdDeviation="8" flood-opacity="0.2"/>
                    </filter>
                    <filter id="grain" x="-20%" y="-20%" width="140%" height="140%">
                        <feTurbulence type="fractalNoise" baseFrequency="0.9" numOctaves="2" seed="7" result="noise"/>
                        <feColorMatrix type="saturate" values="0"/>
                        <feComponentTransfer>
                            <feFuncA type="table" tableValues="0 0 0.08 0.12 0"/>
                        </feComponentTransfer>
                        <feBlend in="SourceGraphic" in2="noise" mode="overlay"/>
                    </filter>
                </defs>
                <g filter="url(#softShadow)">
                    <circle cx="512" cy="512" r="360" fill="none" stroke="url(#gold)" stroke-width="28"/>
                    <circle cx="512" cy="512" r="312" fill="none" stroke="url(#gold)" stroke-width="18"/>
                    <path d="M274,302 A360,360 0 0 1 750,270" fill="none" stroke="url(#gold)" stroke-width="10" stroke-linecap="round" opacity="0.8"/>
                    <path d="M268,762 A360,360 0 0 0 754,744" fill="none" stroke="url(#gold)" stroke-width="10" stroke-linecap="round" opacity="0.8"/>
                    <line x1="260" x2="764" y1="452" y2="452" stroke="url(#gold)" stroke-width="20" stroke-linecap="round"/>
                    <line x1="260" x2="764" y1="572" y2="572" stroke="url(#gold)" stroke-width="20" stroke-linecap="round"/>
                    <text x="512" y="540" text-anchor="middle" dominant-baseline="middle" font-size="180" font-weight="900" letter-spacing="6" fill="url(#gold)" filter="url(#grain)" font-family="Arial, Helvetica, sans-serif">PAGADO</text>
                </g>
            </svg>
        </div>"###
    } else {
        ""
    };
    html = html.replace("{{PAID_SIGNATURE}}", paid_signature);

    html
}
