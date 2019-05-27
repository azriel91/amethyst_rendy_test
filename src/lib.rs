#[cfg(test)]
mod tests {
    use amethyst::{core::TransformBundle, renderer::RenderEmptyBundle};
    use amethyst::{Error, LogLevelFilter, LoggerConfig};
    use amethyst_test::AmethystApplication;

    macro_rules! test_n {
        ($test_name:ident) => {
            #[test]
            fn $test_name() -> Result<(), Error> {
                amethyst::start_logger(LoggerConfig {
                    level_filter: LogLevelFilter::Debug,
                    ..Default::default()
                });

                AmethystApplication::blank()
                    .with_bundle(TransformBundle::new())
                    .with_bundle(RenderEmptyBundle::new())
                    .run() // segfaults
                           // .run_isolated() // doesn't segfault
            }
        };
    }

    test_n!(test_0);
    test_n!(test_1);
    test_n!(test_2);
    test_n!(test_3);
    test_n!(test_4);
    test_n!(test_5);
    test_n!(test_6);
    test_n!(test_7);
    test_n!(test_8);
    test_n!(test_9);

    test_n!(test_10);
    test_n!(test_11);
    test_n!(test_12);
    test_n!(test_13);
    test_n!(test_14);
    test_n!(test_15);
    test_n!(test_16);
    test_n!(test_17);
    test_n!(test_18);
    test_n!(test_19);

    test_n!(test_20);
    test_n!(test_21);
    test_n!(test_22);
    test_n!(test_23);
    test_n!(test_24);
    test_n!(test_25);
    test_n!(test_26);
    test_n!(test_27);
    test_n!(test_28);
    test_n!(test_29);

    test_n!(test_30);
    test_n!(test_31);
    test_n!(test_32);
    test_n!(test_33);
    test_n!(test_34);
    test_n!(test_35);
    test_n!(test_36);
    test_n!(test_37);
    test_n!(test_38);
    test_n!(test_39);

    test_n!(test_40);
    test_n!(test_41);
    test_n!(test_42);
    test_n!(test_43);
    test_n!(test_44);
    test_n!(test_45);
    test_n!(test_46);
    test_n!(test_47);
    test_n!(test_48);
    test_n!(test_49);

    test_n!(test_50);
    test_n!(test_51);
    test_n!(test_52);
    test_n!(test_53);
    test_n!(test_54);
    test_n!(test_55);
    test_n!(test_56);
    test_n!(test_57);
    test_n!(test_58);
    test_n!(test_59);

    test_n!(test_60);
    test_n!(test_61);
    test_n!(test_62);
    test_n!(test_63);
    test_n!(test_64);
    test_n!(test_65);
    test_n!(test_66);
    test_n!(test_67);
    test_n!(test_68);
    test_n!(test_69);

    test_n!(test_70);
    test_n!(test_71);
    test_n!(test_72);
    test_n!(test_73);
    test_n!(test_74);
    test_n!(test_75);
    test_n!(test_76);
    test_n!(test_77);
    test_n!(test_78);
    test_n!(test_79);

    test_n!(test_80);
    test_n!(test_81);
    test_n!(test_82);
    test_n!(test_83);
    test_n!(test_84);
    test_n!(test_85);
    test_n!(test_86);
    test_n!(test_87);
    test_n!(test_88);
    test_n!(test_89);

    test_n!(test_90);
    test_n!(test_91);
    test_n!(test_92);
    test_n!(test_93);
    test_n!(test_94);
    test_n!(test_95);
    test_n!(test_96);
    test_n!(test_97);
    test_n!(test_98);
    test_n!(test_99);
}
