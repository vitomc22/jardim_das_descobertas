#include "art.h"

#include <cmath>
#include <cstdio>
#include <sstream>

namespace jardim::art {

static std::string svg(const std::string& body, const char* vb = "0 0 120 120") {
    return std::string(R"~(<svg xmlns="http://www.w3.org/2000/svg" viewBox=")~") + vb + "\">" + body + "</svg>";
}
static std::string num(double v) { char b[32]; snprintf(b, sizeof b, "%.2f", v); return b; }

static const std::string EYES =
    R"~(<circle cx="48" cy="59" r="3" fill="#283e3b"/><circle cx="72" cy="59" r="3" fill="#283e3b"/><path d="M53 71q7 7 14 0" fill="none" stroke="#283e3b" stroke-width="3" stroke-linecap="round"/>)~";

static std::string sockBody(const std::string& color, const std::string& pattern);
static std::string cupBody(double amount);
static std::string containerBody(const std::string& place);
static std::string towerBody(const std::vector<std::string>& colors);
static std::string clockBody(int hour);

static std::string body(const std::string& kind, const std::string& color) {
    const std::string c = color.empty() ? "#eb6d58" : color;
    if (kind == "circle") return R"~(<circle cx="60" cy="60" r="38" fill=")~" + c + "\"/>";
    if (kind == "square") return R"~(<rect x="23" y="23" width="74" height="74" rx="9" fill=")~" + c + "\"/>";
    if (kind == "triangle") return R"~(<path d="M60 18 105 96H15Z" fill=")~" + c + R"~(" stroke-linejoin="round"/>)~";
    if (kind == "star") return R"~(<path d="m60 12 14 30 33 5-24 24 6 34-29-16-29 16 6-34-24-24 33-5Z" fill=")~" + c + "\"/>";
    if (kind == "apple")
        return R"~(<path d="M60 28q-3-18 16-22-1 19-16 22" fill="#5e9d70"/><path d="M60 34C9 8 13 84 40 101q12 8 20 0 16 10 29-5 32-48 6-64-17-10-35 2" fill=")~" + c +
               R"~("/><path d="M37 45q-9 8-8 20" stroke="#ffc4ad" stroke-width="7" stroke-linecap="round" fill="none"/>)~";
    if (kind == "flower")
        return R"~(<path d="M60 60v53m0-19q-32-21-32-3 14 20 32 9m0-2q29-27 32-7-12 19-32 12" fill="#659976" stroke="#659976" stroke-width="5"/><g fill=")~" + c +
               R"~("><ellipse cx="60" cy="34" rx="17" ry="22"/><ellipse cx="80" cy="51" rx="22" ry="17"/><ellipse cx="72" cy="72" rx="17" ry="22"/><ellipse cx="47" cy="72" rx="17" ry="22"/><ellipse cx="37" cy="51" rx="22" ry="17"/></g><circle cx="60" cy="54" r="19" fill="#ffdf78"/><circle cx="54" cy="52" r="2" fill="#33493a"/><circle cx="66" cy="52" r="2" fill="#33493a"/><path d="M56 60q4 4 8 0" fill="none" stroke="#33493a" stroke-width="2"/>)~";
    if (kind == "bird")
        return R"~(<ellipse cx="57" cy="65" rx="37" ry="32" fill="#83b7cb"/><circle cx="70" cy="41" r="24" fill="#83b7cb"/><path d="m93 42 20 9-20 9" fill="#e9ad46"/><path d="M17 66 4 42 33 56" fill="#6093ae"/><path d="M31 66q24-20 36 3-17 28-36-3" fill="#aed6df"/><circle cx="77" cy="38" r="3" fill="#283e3b"/><path d="m46 95-4 15m26-15 4 15" stroke="#c68e49" stroke-width="5"/>)~";
    if (kind == "sun") {
        std::string rays;
        for (int i = 0; i < 8; ++i) rays += R"~(<path d="M60 9v9" transform="rotate()~" + std::to_string(i * 45) + R"~( 60 60)"/>)~";
        return R"~(<g stroke="#efbd46" stroke-width="6" stroke-linecap="round">)~" + rays + R"~(</g><circle cx="60" cy="60" r="32" fill="#f4ca61"/>)~" + EYES;
    }
    if (kind == "bunny")
        return R"~(<ellipse cx="43" cy="29" rx="13" ry="26" fill="#f4e9d1" transform="rotate(-14 43 29)"/><ellipse cx="77" cy="29" rx="13" ry="26" fill="#f4e9d1" transform="rotate(14 77 29)"/><ellipse cx="43" cy="27" rx="6" ry="17" fill="#edb8a6"/><ellipse cx="77" cy="27" rx="6" ry="17" fill="#edb8a6"/><ellipse cx="60" cy="101" rx="34" ry="25" fill="#e2d4b8"/><ellipse cx="60" cy="67" rx="38" ry="34" fill="#f4e9d1"/>)~" +
               EYES + R"~(<ellipse cx="38" cy="70" rx="7" ry="4" fill="#edb8a6"/><ellipse cx="82" cy="70" rx="7" ry="4" fill="#edb8a6"/>)~";
    if (kind == "cat")
        return R"~(<path d="M27 46 20 14 49 32M71 32 100 14 94 49" fill="#d7a56e"/><ellipse cx="60" cy="65" rx="43" ry="37" fill="#e7bc87"/>)~" + EYES +
               R"~(<path d="m23 64 21 4m-22 9 22-3m53-10-21 4m22 9-22-3" stroke="#8d6c4f" stroke-width="2"/><path d="m55 63 5 5 5-5" fill="#ce8e88"/>)~";
    if (kind == "fish")
        return R"~(<path d="M82 60 112 31v58Z" fill="#d9a75d"/><ellipse cx="52" cy="60" rx="40" ry="28" fill="#efbe6d"/><path d="M53 37 64 17 79 43M54 82 69 102 80 76" fill="#d9a75d"/><circle cx="30" cy="54" r="4" fill="#294b43"/><path d="M16 66q8 6 14 0" stroke="#294b43" stroke-width="2" fill="none"/>)~";
    if (kind == "ball")
        return R"~(<circle cx="60" cy="60" r="42" fill="#7fb8ca"/><path d="M32 28q37 28 55 59M20 68q33-32 80-22" fill="none" stroke="#e6f1ed" stroke-width="9"/><path d="M60 18q-13 37-10 84" stroke="#d3e9de" stroke-width="5" fill="none"/>)~";
    if (kind == "moon")
        return R"~(<path d="M78 14a44 44 0 1 0 26 67Q45 92 78 14" fill="#edc976"/><circle cx="40" cy="74" r="4" fill="#d8b263"/><circle cx="45" cy="44" r="6" fill="#d8b263"/>)~";
    if (kind == "house")
        return R"~(<path d="M23 49h74v58H23Z" fill="#e9c69b"/><path d="M10 52 60 9l50 43Z" fill="#c77e69"/><rect x="46" y="70" width="26" height="37" rx="3" fill="#73a493"/><rect x="30" y="59" width="14" height="15" rx="2" fill="#faf4da"/><rect x="77" y="59" width="14" height="15" rx="2" fill="#faf4da"/>)~";
    if (kind == "book")
        return R"~(<path d="M60 34Q32 16 10 28v69q25-10 50 6 25-16 50-6V28Q87 16 60 34" fill="#f8e4b5" stroke="#bc9a65" stroke-width="4"/><path d="M60 34v69M24 43l23 5m-23 8 23 5m-23 8 23 5m27-26 23-5m-23 18 23-5m-23 18 23-5" fill="none" stroke="#bc9a65" stroke-width="3"/>)~";
    if (kind == "letters")
        return R"~(<text x="10" y="84" font-family="serif" font-weight="bold" font-size="75" fill="#8e729e">Aa</text>)~";
    if (kind == "clock") return clockBody(3);
    if (kind == "socks") return sockBody("#83b7cb", "stripes");
    if (kind == "cup") return cupBody(1.0);
    if (kind == "tower") return towerBody({"#5799be", "#68a585"});
    if (kind == "container") return containerBody("inside");
    if (kind == "colorpath")
        return R"~(<circle cx="25" cy="60" r="20" fill="#e87961"/><circle cx="60" cy="60" r="20" fill="#5799be"/><circle cx="95" cy="60" r="20" fill="#e87961"/>)~";
    if (kind == "leaf")
        return R"~(<path d="M60 103V45" stroke="#347665" stroke-width="7" stroke-linecap="round"/><path d="M59 70Q8 72 15 25q46-8 44 45" fill="#7cb78f"/><path d="M60 51q-5-43 43-41 7 43-43 41" fill="#47977e"/>)~";
    return body("star", "#efbd46");
}

std::string icon(const std::string& kind, const std::string& color) { return svg(body(kind, color)); }

static std::string sockBody(const std::string& color, const std::string& pattern) {
    std::string marks = pattern == "dots"
        ? R"~(<circle cx="61" cy="52" r="4"/><circle cx="77" cy="66" r="4"/><circle cx="61" cy="81" r="4"/>)~"
        : R"~(<path d="M48 48h37M48 64h37M48 80h32" stroke="#fff7de" stroke-width="6"/>)~";
    return R"~(<path d="M45 15h42v64q0 28-30 29H30q-23 0-15-19 3-8 17-10l13-2Z" fill=")~" + color +
           R"~(" stroke="#607f80" stroke-width="2"/><path d="M45 15h42v14H45Z" fill="#fff7de"/><g fill="#fff7de">)~" + marks + "</g>";
}
std::string sock(const std::string& color, const std::string& pattern) { return svg(sockBody(color, pattern)); }

static std::string cupBody(double amount) {
    double top = 91.0 - amount * 64.0;
    std::string water;
    if (amount > 0)
        water = "<path d=\"M" + num(31.0 + (top - 17.0) * 7.0 / 84.0) + " " + num(top) + "h" + num(58.0 - (top - 17.0) * 14.0 / 84.0) +
                R"~(L80 98H40Z" fill="#8fc5db"/>)~";
    return R"~(<path d="M31 17h58l-7 84H38Z" fill="#f3f7f2" stroke="#709a97" stroke-width="4"/>)~" + water +
           R"~(<path d="M46 31v25" stroke="#fff" stroke-width="5" stroke-linecap="round"/>)~";
}
std::string cup(double amount) { return svg(cupBody(amount)); }

static std::string miniFlower(double x, double y, double size) {
    return "<g transform=\"translate(" + num(x) + " " + num(y) + ") scale(" + num(size / 120.0) + ")\">" + body("flower", "") + "</g>";
}
static std::string containerBody(const std::string& place) {
    bool in = place == "inside";
    return R"~(<rect x="9" y="32" width="73" height="72" rx="8" fill="#f1dfbd" stroke="#bb9a6b" stroke-width="3"/><path d="M14 47h63" stroke="#d3b78c" stroke-width="2"/>)~" +
           miniFlower(in ? 43 : 89, in ? 47 : 49, 29);
}
std::string container(const std::string& place) { return svg(containerBody(place)); }

static std::string towerBody(const std::vector<std::string>& colors) {
    std::string s;
    double n = double(colors.size());
    for (size_t i = 0; i < colors.size(); ++i)
        s += "<rect x=\"" + num(20 + i * 4.0) + "\" y=\"" + num((120 + (n - 2) * 27) / 2 - i * 27.0) + "\" width=\"" + num(80 - i * 8.0) +
             "\" height=\"24\" rx=\"5\" fill=\"" + colors[i] + R"~(" stroke="#fff7e6" stroke-width="2"/>)~";
    return s;
}
std::string tower(const std::vector<std::string>& colors) { return svg(towerBody(colors)); }

std::string position(const std::string& dir) {
    double x = 92, y = 46;
    if (dir == "above") { x = 46; y = 0; } else if (dir == "below") { x = 46; y = 92; } else if (dir == "left") { x = 0; y = 46; }
    return svg(R"~(<rect x="40" y="40" width="40" height="40" rx="5" fill="#debd8c"/><path d="M40 50h40m-20-10v40" stroke="#c4a474" stroke-width="2"/>)~" + miniFlower(x, y, 28));
}

static std::string clockBody(int hour) {
    double angle = hour * M_PI / 6;
    std::string nums;
    for (int i = 0; i < 12; ++i) {
        double a = (i + 1) * M_PI / 6;
        nums += "<text x=\"" + num(60 + 43 * std::sin(a)) + "\" y=\"" + num(64 - 43 * std::cos(a)) +
                R"~(" text-anchor="middle" font-size="11" font-family="sans-serif" fill="#46625c">)~" + std::to_string(i + 1) + "</text>";
    }
    return R"~(<circle cx="60" cy="60" r="56" fill="#fffaf0" stroke="#92b4ad" stroke-width="5"/>)~" + nums +
           R"~(<path d="M60 60V27" stroke="#659a8c" stroke-width="4" stroke-linecap="round"/><path d="M60 60L)~" + num(60 + 25 * std::sin(angle)) + " " +
           num(60 - 25 * std::cos(angle)) + R"~(" stroke="#294b43" stroke-width="6" stroke-linecap="round"/><circle cx="60" cy="60" r="4" fill="#294b43"/>)~";
}
std::string clock(int hour) { return svg(clockBody(hour)); }

std::string garden() {
    return svg(R"~(<circle cx="397" cy="68" r="42" fill="#f8d47c"/><path d="M57 63q10-26 31-10 20-34 40 0 23-8 28 14H57" fill="#fff" opacity=".8"/><path d="M7 256Q148 136 285 239q144-100 235-22v83H7" fill="#b7d4b1"/><path d="M2 290q142-89 278-19 147-62 240-13v42H2" fill="#8fba96"/><path d="M332 248v-98" stroke="#8c9a73" stroke-width="14"/><ellipse cx="332" cy="119" rx="61" ry="80" fill="#6d9d7b"/><ellipse cx="300" cy="125" rx="22" ry="40" fill="#7aab86"/><g transform="translate(160 130) scale(1.25)">)~" +
                   body("bunny", "") + R"~(</g><g transform="translate(63 161) scale(.7)">)~" + body("flower", "") +
                   R"~(</g><g transform="translate(396 179) scale(.65)">)~" + body("bird", "") + R"~(</g><g transform="translate(352 239) scale(.45)">)~" +
                   body("flower", "#f4cd64") + "</g>",
               "0 0 520 300");
}

std::string tile(const std::string& kind) {
    for (const char* s : {"flower", "letters", "book", "clock", "socks", "cup", "tower", "container", "colorpath"})
        if (kind == s) return icon(kind);
    std::string a = "leaf", b = "sun";
    if (kind == "shapes") { a = "circle"; b = "triangle"; }
    else if (kind == "count") { a = "apple"; b = "apple"; }
    else if (kind == "memory") { a = "bird"; b = "bird"; }
    else if (kind == "pattern") { a = "circle"; b = "star"; }
    else if (kind == "odd") { a = "circle"; b = "square"; }
    else if (kind == "sum") { a = "apple"; b = "star"; }
    else if (kind == "order") { a = "triangle"; b = "star"; }
    std::string ca = kind == "pattern" ? "#8c7db3" : "#edab69", cb = kind == "pattern" ? "#8c7db3" : "#62a58e";
    return svg(body(a, ca) + "<g transform=\"translate(120 0)\">" + body(b, cb) + "</g>", "0 0 240 120");
}

static std::vector<std::string> split(const std::string& s, char sep) {
    std::vector<std::string> out; std::stringstream ss(s); std::string part;
    while (std::getline(ss, part, sep)) out.push_back(part);
    return out;
}
static std::string hex(const std::string& h) { return h.empty() || h == "-" ? "" : "#" + h; }

std::string fromSpec(const std::string& spec) {
    auto p = split(spec, '/');
    if (p.empty()) return icon("star");
    const std::string& k = p[0];
    auto at = [&](size_t i) { return i < p.size() ? p[i] : std::string(); };
    if (k == "icon") return icon(at(1), hex(at(2)));
    if (k == "tile") return tile(at(1));
    if (k == "garden") return garden();
    if (k == "sock") return sock(hex(at(1)), at(2));
    if (k == "cup") return cup(std::atof(at(1).c_str()));
    if (k == "container") return container(at(1));
    if (k == "position") return position(at(1));
    if (k == "clock") return clock(std::atoi(at(1).c_str()));
    if (k == "tower") { std::vector<std::string> cs; for (auto& c : split(at(1), ',')) cs.push_back(hex(c)); return tower(cs); }
    return icon("star");
}

}  // namespace jardim::art
