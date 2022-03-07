use gtk::prelude::*;
use gtk::*;
use log::info;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    info!("Starting App");

    let application = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    application.connect_activate(move |app| {
        // Load the compiled resource bundle
        let resources_bytes = include_bytes!("../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);

        // Load the CSS
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/example/Example/style.css");
        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        info!("{:?}", res);

        // Load the window UI
        let builder = Builder::from_resource("/org/example/Example/main_window.glade");
        //let builder = Builder::from_file("resources/main_window.glade");

        info!("{:?}", builder);

        info!("# of objects: {}", builder.objects().len());
        let mut count = 1;
        for obj in builder.objects() {
            info!(
                "{}: {:?} ",
                count,
                obj
            );
            count += 1;        
        }

        // Get a reference to the window
        let win_name = "main_window";
        let msg = &*format!("Couldn't get window: {}", &win_name);
        let window : ApplicationWindow = builder.object(win_name).expect(msg);
        info!("{:?}", win_name);

        window.set_application(Some(app));

        // Show the UI
        info!("Showing window");
        window.show_all();
    });

    // Run the application and start the event loop
    info!("Launching app and processing event loop");
    application.run();
}

