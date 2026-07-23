#![allow(dead_code)]
// Pandora Logo Scene — visual representation of the Pandora cat + magical box.
//
// This is a data model describing the official Pandora logo:
// a spectral white cat hugging a purple magical box with a gold-accented
// crystal on top and an open purple eye in the center.
//
// Run: cargo run --example logo_scene -p pandora-types

#[derive(Debug)]
enum Color {
    White,
    Purple,
    Gold,
    DarkOutline,
}

#[derive(Debug)]
struct Crystal {
    color: Color,
    facets: u8,
}

#[derive(Debug)]
struct Eye {
    iris_color: Color,
    is_open: bool,
}

#[derive(Debug)]
struct MagicalBox {
    primary_color: Color,
    accent_color: Color,
    topper: Crystal,
    centerpiece: Eye,
}

#[derive(Debug)]
struct Cat {
    fill_color: Color,
    outline: Color,
    is_hugging_object: bool,
    tail_posture: String,
}

#[derive(Debug)]
struct ImageScene {
    file_name: String,
    subject: Cat,
    prop: MagicalBox,
    layout: String,
}

fn main() {
    let top_crystal = Crystal {
        color: Color::Purple,
        facets: 6,
    };

    let center_eye = Eye {
        iris_color: Color::Purple,
        is_open: true,
    };

    let ornate_box = MagicalBox {
        primary_color: Color::Purple,
        accent_color: Color::Gold,
        topper: top_crystal,
        centerpiece: center_eye,
    };

    let spectral_cat = Cat {
        fill_color: Color::White,
        outline: Color::DarkOutline,
        is_hugging_object: true,
        tail_posture: String::from("Curled underneath the box"),
    };

    let rendered_scene = ImageScene {
        file_name: String::from("13056 (1).png"),
        subject: spectral_cat,
        prop: ornate_box,
        layout: String::from(
            "Cat is draped over the top and sides of the box, looking forward, face blank.",
        ),
    };

    println!("{:#?}", rendered_scene);
}
