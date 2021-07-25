use gtk::prelude::*;
use gtk::{
    self, Application, ApplicationWindow, Button,
    Orientation, Label, HeaderBar, ComboBoxText,
    Entry, glib
};
use zero_pass_backend::{ self as zpb, encrypt, CipherResult };

fn main() {
   let app = Application::new(Some("io.github.caiovieiraf.zero-pass-app"), Default::default());
   app.connect_activate(render);
   app.run();
}

fn render(app: &Application){
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Zero Pass")
        .show_menubar(false)
        .default_height(80)
        .default_width(60)
        .startup_id("zero-pass-app")
        .build();

    let controls = HeaderBar::new();

    let padding = 20;

    let methods_list: Vec<String> = zpb::get_methods().keys().cloned().collect();

    let methods_menu = ComboBoxText::new();
    for i in methods_list.iter(){
        methods_menu.append_text(i);
    }

    let label = Label::new(Some("Entre com as informações necessárias:"));
    let result = Label::builder()
        .label("O resultado aparecerá aqui.")
        .selectable(true)
        .build();

    let input_unique = build_button("Palavra unica aqui...");
    let input_variable = build_button("Palavra variável aqui...");
    let button_confirm = Button::with_label("Gerar senha");

    button_confirm.connect_clicked(glib::clone!(
            @weak input_unique, @weak input_variable, @weak result, @weak methods_menu  => move |_| {
                let iu = input_unique.buffer().text().clone();
                let iv = input_variable.buffer().text().clone();
                let choice = methods_menu.active_text();

                let method_args = encrypt::MethodArgs { word: iu.as_str(), password: iv.as_str() };

                match choice {
                    Some(i) => {
                        result.set_label(
                            match &encrypt_input(
                                &zpb::get_methods().get(i.as_str()).unwrap()(method_args)
                            ) {
                                Ok(s) => s,
                                Err(e) => {
                                    println!("{:?}: O caracter inserido é inválido.", e);
                                    "O caracter inserido é inválido."
                                }
                            }
                        );
                    },
                    None => {result.set_label("É nescessário especificar um método de criptografia")}
                }
        }
    ));

    let app_area = gtk::Box::new(Orientation::Vertical, 0);

    let main_area = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(padding)
        .margin_top(padding)
        .margin_start(padding)
        .margin_end(padding)
        .build();

    window.set_child(Some(&app_area));

    app_area.append(&controls);
    app_area.append(&main_area);

    main_area.append(&label);
    main_area.append(&input_unique);
    main_area.append(&input_variable);
    main_area.append(&methods_menu);
    main_area.append(&result);
    main_area.append(&button_confirm);

    window.present();
}

fn encrypt_input(method: &encrypt::Methods) -> CipherResult{
   encrypt::gen_pass(method)
}

fn build_button(placeholder: &str) -> Entry{
    let padding = 20;
    Entry::builder()
        .placeholder_text(placeholder)
        .margin_start(padding)
        .margin_end(padding)
        .build()
}
