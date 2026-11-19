//! Port de `art.js`: os mesmos desenhos vetoriais, gerados como SVG.
//! Ao contrário do WebView, cada SVG é rasterizado UMA vez (resvg/tiny-skia,
//! em paralelo) e vira textura na GPU; depois disso desenhar custa um quad.

fn svg(body: &str) -> String {
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120">{body}</svg>"#)
}

const EYES: &str = r##"<circle cx="48" cy="59" r="3" fill="#283e3b"/><circle cx="72" cy="59" r="3" fill="#283e3b"/><path d="M53 71q7 7 14 0" fill="none" stroke="#283e3b" stroke-width="3" stroke-linecap="round"/>"##;

/// Corpo (sem `<svg>`) de cada ícone, para compor dentro de outros desenhos.
pub fn icon_body(kind: &str, color: Option<&str>) -> String {
    let c = color.unwrap_or("#eb6d58");
    match kind {
        "circle" => format!(r#"<circle cx="60" cy="60" r="38" fill="{c}"/>"#),
        "square" => format!(r#"<rect x="23" y="23" width="74" height="74" rx="9" fill="{c}"/>"#),
        "triangle" => format!(r#"<path d="M60 18 105 96H15Z" fill="{c}" stroke-linejoin="round"/>"#),
        "star" => format!(r#"<path d="m60 12 14 30 33 5-24 24 6 34-29-16-29 16 6-34-24-24 33-5Z" fill="{c}"/>"#),
        "apple" => format!(
            r##"<path d="M60 28q-3-18 16-22-1 19-16 22" fill="#5e9d70"/><path d="M60 34C9 8 13 84 40 101q12 8 20 0 16 10 29-5 32-48 6-64-17-10-35 2" fill="{c}"/><path d="M37 45q-9 8-8 20" stroke="#ffc4ad" stroke-width="7" stroke-linecap="round" fill="none"/>"##
        ),
        "flower" => format!(
            r##"<path d="M60 60v53m0-19q-32-21-32-3 14 20 32 9m0-2q29-27 32-7-12 19-32 12" fill="#659976" stroke="#659976" stroke-width="5"/><g fill="{c}"><ellipse cx="60" cy="34" rx="17" ry="22"/><ellipse cx="80" cy="51" rx="22" ry="17"/><ellipse cx="72" cy="72" rx="17" ry="22"/><ellipse cx="47" cy="72" rx="17" ry="22"/><ellipse cx="37" cy="51" rx="22" ry="17"/></g><circle cx="60" cy="54" r="19" fill="#ffdf78"/><circle cx="54" cy="52" r="2" fill="#33493a"/><circle cx="66" cy="52" r="2" fill="#33493a"/><path d="M56 60q4 4 8 0" fill="none" stroke="#33493a" stroke-width="2"/>"##
        ),
        "bird" => r##"<ellipse cx="57" cy="65" rx="37" ry="32" fill="#83b7cb"/><circle cx="70" cy="41" r="24" fill="#83b7cb"/><path d="m93 42 20 9-20 9" fill="#e9ad46"/><path d="M17 66 4 42 33 56" fill="#6093ae"/><path d="M31 66q24-20 36 3-17 28-36-3" fill="#aed6df"/><circle cx="77" cy="38" r="3" fill="#283e3b"/><path d="m46 95-4 15m26-15 4 15" stroke="#c68e49" stroke-width="5"/>"##.into(),
        "sun" => {
            let rays: String = (0..8).map(|i| format!(r#"<path d="M60 9v9" transform="rotate({} 60 60)"/>"#, i * 45)).collect();
            format!(r##"<g stroke="#efbd46" stroke-width="6" stroke-linecap="round">{rays}</g><circle cx="60" cy="60" r="32" fill="#f4ca61"/>{EYES}"##)
        }
        "bunny" => format!(
            r##"<ellipse cx="43" cy="29" rx="13" ry="26" fill="#f4e9d1" transform="rotate(-14 43 29)"/><ellipse cx="77" cy="29" rx="13" ry="26" fill="#f4e9d1" transform="rotate(14 77 29)"/><ellipse cx="43" cy="27" rx="6" ry="17" fill="#edb8a6"/><ellipse cx="77" cy="27" rx="6" ry="17" fill="#edb8a6"/><ellipse cx="60" cy="101" rx="34" ry="25" fill="#e2d4b8"/><ellipse cx="60" cy="67" rx="38" ry="34" fill="#f4e9d1"/>{EYES}<ellipse cx="38" cy="70" rx="7" ry="4" fill="#edb8a6"/><ellipse cx="82" cy="70" rx="7" ry="4" fill="#edb8a6"/>"##
        ),
        "cat" => format!(
            r##"<path d="M27 46 20 14 49 32M71 32 100 14 94 49" fill="#d7a56e"/><ellipse cx="60" cy="65" rx="43" ry="37" fill="#e7bc87"/>{EYES}<path d="m23 64 21 4m-22 9 22-3m53-10-21 4m22 9-22-3" stroke="#8d6c4f" stroke-width="2"/><path d="m55 63 5 5 5-5" fill="#ce8e88"/>"##
        ),
        "fish" => r##"<path d="M82 60 112 31v58Z" fill="#d9a75d"/><ellipse cx="52" cy="60" rx="40" ry="28" fill="#efbe6d"/><path d="M53 37 64 17 79 43M54 82 69 102 80 76" fill="#d9a75d"/><circle cx="30" cy="54" r="4" fill="#294b43"/><path d="M16 66q8 6 14 0" stroke="#294b43" stroke-width="2" fill="none"/>"##.into(),
        "ball" => r##"<circle cx="60" cy="60" r="42" fill="#7fb8ca"/><path d="M32 28q37 28 55 59M20 68q33-32 80-22" fill="none" stroke="#e6f1ed" stroke-width="9"/><path d="M60 18q-13 37-10 84" stroke="#d3e9de" stroke-width="5" fill="none"/>"##.into(),
        "moon" => r##"<path d="M78 14a44 44 0 1 0 26 67Q45 92 78 14" fill="#edc976"/><circle cx="40" cy="74" r="4" fill="#d8b263"/><circle cx="45" cy="44" r="6" fill="#d8b263"/>"##.into(),
        "house" => r##"<path d="M23 49h74v58H23Z" fill="#e9c69b"/><path d="M10 52 60 9l50 43Z" fill="#c77e69"/><rect x="46" y="70" width="26" height="37" rx="3" fill="#73a493"/><rect x="30" y="59" width="14" height="15" rx="2" fill="#faf4da"/><rect x="77" y="59" width="14" height="15" rx="2" fill="#faf4da"/>"##.into(),
        "book" => r##"<path d="M60 34Q32 16 10 28v69q25-10 50 6 25-16 50-6V28Q87 16 60 34" fill="#f8e4b5" stroke="#bc9a65" stroke-width="4"/><path d="M60 34v69M24 43l23 5m-23 8 23 5m-23 8 23 5m27-26 23-5m-23 18 23-5m-23 18 23-5" fill="none" stroke="#bc9a65" stroke-width="3"/>"##.into(),
        "letters" => r##"<text x="10" y="84" font-family="serif" font-weight="bold" font-size="75" fill="#8e729e">Aa</text>"##.into(),
        "clock" => clock_body(3),
        "socks" => sock_body("#83b7cb", "stripes"),
        "cup" => cup_body(1.0),
        "tower" => tower_body(&["#5799be", "#68a585"]),
        "container" => container_body("inside"),
        "colorpath" => r##"<circle cx="25" cy="60" r="20" fill="#e87961"/><circle cx="60" cy="60" r="20" fill="#5799be"/><circle cx="95" cy="60" r="20" fill="#e87961"/>"##.into(),
        "leaf" => r##"<path d="M60 103V45" stroke="#347665" stroke-width="7" stroke-linecap="round"/><path d="M59 70Q8 72 15 25q46-8 44 45" fill="#7cb78f"/><path d="M60 51q-5-43 43-41 7 43-43 41" fill="#47977e"/>"##.into(),
        _ => icon_body("star", Some("#efbd46")),
    }
}

pub fn icon(kind: &str, color: Option<&str>) -> String {
    svg(&icon_body(kind, color))
}

fn sock_body(color: &str, pattern: &str) -> String {
    let marks = if pattern == "dots" {
        r#"<circle cx="61" cy="52" r="4"/><circle cx="77" cy="66" r="4"/><circle cx="61" cy="81" r="4"/>"#
    } else {
        r##"<path d="M48 48h37M48 64h37M48 80h32" stroke="#fff7de" stroke-width="6"/>"##
    };
    format!(
        r##"<path d="M45 15h42v64q0 28-30 29H30q-23 0-15-19 3-8 17-10l13-2Z" fill="{color}" stroke="#607f80" stroke-width="2"/><path d="M45 15h42v14H45Z" fill="#fff7de"/><g fill="#fff7de">{marks}</g>"##
    )
}
pub fn sock(color: &str, pattern: &str) -> String {
    svg(&sock_body(color, pattern))
}

fn cup_body(amount: f64) -> String {
    let top = 91.0 - amount * 64.0;
    let water = if amount > 0.0 {
        format!(
            r##"<path d="M{} {top}h{}L80 98H40Z" fill="#8fc5db"/>"##,
            31.0 + (top - 17.0) * 7.0 / 84.0,
            58.0 - (top - 17.0) * 14.0 / 84.0
        )
    } else {
        String::new()
    };
    format!(
        r##"<path d="M31 17h58l-7 84H38Z" fill="#f3f7f2" stroke="#709a97" stroke-width="4"/>{water}<path d="M46 31v25" stroke="#fff" stroke-width="5" stroke-linecap="round"/>"##
    )
}
pub fn cup(amount: f64) -> String {
    svg(&cup_body(amount))
}

/// Flor em miniatura posicionada (equivale ao `<svg x y width height>` aninhado).
fn mini_flower(x: f64, y: f64, size: f64) -> String {
    let s = size / 120.0;
    format!(r#"<g transform="translate({x} {y}) scale({s})">{}</g>"#, icon_body("flower", None))
}

fn container_body(place: &str) -> String {
    let (x, y) = if place == "inside" { (43.0, 47.0) } else { (89.0, 49.0) };
    format!(
        r##"<rect x="9" y="32" width="73" height="72" rx="8" fill="#f1dfbd" stroke="#bb9a6b" stroke-width="3"/><path d="M14 47h63" stroke="#d3b78c" stroke-width="2"/>{}"##,
        mini_flower(x, y, 29.0)
    )
}
pub fn container(place: &str) -> String {
    svg(&container_body(place))
}

fn tower_body(colors: &[&str]) -> String {
    let n = colors.len() as f64;
    colors
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let i = i as f64;
            format!(
                r##"<rect x="{}" y="{}" width="{}" height="24" rx="5" fill="{c}" stroke="#fff7e6" stroke-width="2"/>"##,
                20.0 + i * 4.0,
                (120.0 + (n - 2.0) * 27.0) / 2.0 - i * 27.0,
                80.0 - i * 8.0
            )
        })
        .collect()
}
pub fn tower(colors: &[&str]) -> String {
    svg(&tower_body(colors))
}

pub fn position(dir: &str) -> String {
    let (x, y) = match dir {
        "above" => (46.0, 0.0),
        "below" => (46.0, 92.0),
        "left" => (0.0, 46.0),
        _ => (92.0, 46.0),
    };
    svg(&format!(
        r##"<rect x="40" y="40" width="40" height="40" rx="5" fill="#debd8c"/><path d="M40 50h40m-20-10v40" stroke="#c4a474" stroke-width="2"/>{}"##,
        mini_flower(x, y, 28.0)
    ))
}

fn clock_body(hour: u32) -> String {
    let angle = f64::from(hour) * std::f64::consts::PI / 6.0;
    let nums: String = (0..12)
        .map(|i| {
            let a = f64::from(i + 1) * std::f64::consts::PI / 6.0;
            format!(
                r##"<text x="{:.2}" y="{:.2}" text-anchor="middle" font-size="11" font-family="sans-serif" fill="#46625c">{}</text>"##,
                60.0 + 43.0 * a.sin(),
                64.0 - 43.0 * a.cos(),
                i + 1
            )
        })
        .collect();
    format!(
        r##"<circle cx="60" cy="60" r="56" fill="#fffaf0" stroke="#92b4ad" stroke-width="5"/>{nums}<path d="M60 60V27" stroke="#659a8c" stroke-width="4" stroke-linecap="round"/><path d="M60 60L{:.2} {:.2}" stroke="#294b43" stroke-width="6" stroke-linecap="round"/><circle cx="60" cy="60" r="4" fill="#294b43"/>"##,
        60.0 + 25.0 * angle.sin(),
        60.0 - 25.0 * angle.cos()
    )
}
pub fn clock(hour: u32) -> String {
    svg(&clock_body(hour))
}

pub fn garden() -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 520 300"><circle cx="397" cy="68" r="42" fill="#f8d47c"/><path d="M57 63q10-26 31-10 20-34 40 0 23-8 28 14H57" fill="#fff" opacity=".8"/><path d="M7 256Q148 136 285 239q144-100 235-22v83H7" fill="#b7d4b1"/><path d="M2 290q142-89 278-19 147-62 240-13v42H2" fill="#8fba96"/><path d="M332 248v-98" stroke="#8c9a73" stroke-width="14"/><ellipse cx="332" cy="119" rx="61" ry="80" fill="#6d9d7b"/><ellipse cx="300" cy="125" rx="22" ry="40" fill="#7aab86"/><g transform="translate(160 130) scale(1.25)">{}</g><g transform="translate(63 161) scale(.7)">{}</g><g transform="translate(396 179) scale(.65)">{}</g><g transform="translate(352 239) scale(.45)">{}</g></svg>"##,
        icon_body("bunny", None),
        icon_body("flower", None),
        icon_body("bird", None),
        icon_body("flower", Some("#f4cd64"))
    )
}

/// `Art.tile(kind)`: ícone do cartão. Os pares viram um único SVG 240×120.
pub fn tile(kind: &str) -> String {
    const SINGLE: [&str; 9] = ["flower", "letters", "book", "clock", "socks", "cup", "tower", "container", "colorpath"];
    if SINGLE.contains(&kind) {
        return icon(kind, None);
    }
    let (a, b) = match kind {
        "shapes" => ("circle", "triangle"),
        "count" => ("apple", "apple"),
        "memory" => ("bird", "bird"),
        "pattern" => ("circle", "star"),
        "odd" => ("circle", "square"),
        "sum" => ("apple", "star"),
        "order" => ("triangle", "star"),
        _ => ("leaf", "sun"),
    };
    let (ca, cb) = if kind == "pattern" { ("#8c7db3", "#8c7db3") } else { ("#edab69", "#62a58e") };
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 240 120">{}<g transform="translate(120 0)">{}</g></svg>"#,
        icon_body(a, Some(ca)),
        icon_body(b, Some(cb))
    )
}
