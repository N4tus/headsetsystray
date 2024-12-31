use bitflags::bitflags;
use ksni::blocking::TrayMethods;

bitflags! {
    struct HeadestToggle: u32 {
        const SIDETONE = 0b01;
        const VOICEPROMT = 0b10;
    }
}

struct MyTray {
    flags: HeadestToggle,
}

impl MyTray {
    fn command(&self, flag: HeadestToggle, p: &str) {
        let mut c = std::process::Command::new("headsetcontrol");
        if self.flags.intersects(flag) {
            c.args([p, "1"]);
        } else {
            c.args([p, "0"]);
        }
        _ = c.spawn();
    }

    fn toggle_sidetone(&mut self) {
        self.flags.toggle(HeadestToggle::SIDETONE);
        self.command(HeadestToggle::SIDETONE, "-s");
    }

    fn toggle_voicepromt(&mut self) {
        self.flags.toggle(HeadestToggle::VOICEPROMT);
        self.command(HeadestToggle::VOICEPROMT, "-v");
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
        if self.flags.intersects(HeadestToggle::SIDETONE) {
            "Headset Control (active)"
        } else {
            "Headset Control (inactive)"
        }
        .into()
    }
    fn activate(&mut self, _x: i32, _y: i32) {
        self.toggle_sidetone();
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            CheckmarkItem {
                label: "Sidetone".into(),
                checked: self.flags.intersects(HeadestToggle::SIDETONE),
                activate: Box::new(Self::toggle_sidetone),
                ..Default::default()
            }
            .into(),
            CheckmarkItem {
                label: "Voice Promt".into(),
                checked: self.flags.intersects(HeadestToggle::VOICEPROMT),
                activate: Box::new(Self::toggle_voicepromt),
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
    let tray = MyTray {
        flags: HeadestToggle::empty(),
    };
    let _handle = tray.spawn().unwrap();

    loop {
        std::thread::park()
    }
}
