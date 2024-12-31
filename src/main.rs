use ksni::blocking::TrayMethods;

#[derive(Debug)]
struct MyTray {
    active: bool,
}

impl MyTray {
    fn toggle(&mut self) {
        self.active = !self.active;
        let mut c = std::process::Command::new("headsetcontrol");
        if self.active {
            c.args(["-s", "1"]);
        } else {
            c.args(["-s", "0"]);
        }
        _ = c.spawn();
    }
}

impl ksni::Tray for MyTray {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn icon_name(&self) -> String {
        "headset".into()
    }
    fn title(&self) -> String {
        if self.active {
            "Headset Control (active)"
        } else {
            "Headset Control (inactive)"
        }
        .into()
    }
    fn activate(&mut self, _x: i32, _y: i32) {
        self.toggle();
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            CheckmarkItem {
                label: "Active".into(),
                checked: self.active,
                activate: Box::new(Self::toggle),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

fn main() {
    let tray = MyTray { active: false };
    let _handle = tray.spawn().unwrap();

    loop {
        std::thread::park()
    }
}
