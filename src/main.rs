use leptos::{logging::log, *};

#[component]
fn App() -> impl IntoView {
    view! {
        <Input/>
    }
}

#[component]
fn Input() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let converted = move || convert_en_ar(name());

    view! {
        <input type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p>"Name is: " {converted}</p>
    }
}

fn convert_en_ar(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();
    log!("Input: {}", chars.iter().collect::<String>());
    let mut converted = Vec::<char>::new();
    for char in chars.iter() {
        match char {
            ' ' => converted.push(' '),
            'A' => converted.push('ا'),
            'b' => converted.push('ب'),
            't' => converted.push('ت'),
            'V' => converted.push('ث'),
            'j' => converted.push('ج'),
            'H' => converted.push('ح'),
            'x' => converted.push('خ'),
            'd' => converted.push('د'),
            '*' => converted.push('ذ'),
            'r' => converted.push('ر'),
            'z' => converted.push('ز'),
            's' => converted.push('س'),
            '$' => converted.push('ش'),
            'S' => converted.push('ص'),
            'D' => converted.push('ض'),
            'T' => converted.push('ط'),
            'Z' => converted.push('ظ'),
            'E' => converted.push('ع'),
            'g' => converted.push('غ'),
            'f' => converted.push('ف'),
            'q' => converted.push('ق'),
            'k' => converted.push('ك'),
            'l' => converted.push('ل'),
            'm' => converted.push('م'),
            'n' => converted.push('ن'),
            'h' => converted.push('ه'),
            'w' => converted.push('و'),
            'y' => converted.push('ي'),
            'Y' => converted.push('ى'),
            _ => log!("NOT IMPLEMENTED"),
        };
    }
    leptos::logging::log!("Ouput: {}", converted.iter().collect::<String>());
    converted.iter().collect::<String>()
}

fn main() {
    leptos::mount_to_body(App)
}
