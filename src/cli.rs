use clap::App;

pub fn build_cli() -> App<'static, 'static> {
    clap_app!(pbctl =>
        (version: "0.1")
        (author: "Martin Habovštiak <martin.habovstiak@gmail.com>")
        (about: "Controls lights")
        (@subcommand lights =>
            (about: "Controls lights")
            (@subcommand on =>
                (about: "Turns on all lights")
            )
            (@subcommand off =>
                (about: "Turns off all lights")
            )
            (@subcommand main =>
                (about: "Controls lights in main room")
                (@subcommand on =>
                    (about: "Turns on all lights")
                )
                (@subcommand off =>
                    (about: "Turns off all lights")
                )
                (@subcommand front =>
                    (about: "Controls front lights in main room")
                    (@subcommand on =>
                        (about: "Turns on the lights")
                    )
                    (@subcommand off =>
                        (about: "Turns off the lights")
                    )
                )
                (@subcommand back =>
                    (about: "Controls back lights in main room")
                    (@subcommand on =>
                        (about: "Turns on the lights")
                    )
                    (@subcommand off =>
                        (about: "Turns off the lights")
                    )
                )
            )
            (@subcommand lab =>
                (about: "Controls lights in the lab")
                (@subcommand on =>
                    (about: "Turns on all lights")
                )
                (@subcommand off =>
                    (about: "Turns off all lights")
                )
                (@subcommand left =>
                    (about: "Controls left lights in the lab")
                    (@subcommand on =>
                        (about: "Turns on the lights")
                    )
                    (@subcommand off =>
                        (about: "Turns off the lights")
                    )
                )
                (@subcommand right =>
                    (about: "Controls right lights in the lab")
                    (@subcommand on =>
                        (about: "Turns on the lights")
                    )
                    (@subcommand off =>
                        (about: "Turns off the lights")
                    )
                )
            )
        )
    )
}
