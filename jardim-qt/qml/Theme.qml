pragma Singleton
import QtQuick

// Paleta e tipografia do style.css original.
QtObject {
    readonly property color ink: "#294b43"
    readonly property color muted: "#6d7d70"
    readonly property color paper: "#f8f6ee"
    readonly property color green: "#347b65"
    readonly property color greenShadow: "#26614f"
    readonly property color line: "#e2e6d9"
    readonly property color quiet: "#eeeee4"
    readonly property color quietBorder: "#dedfd2"
    readonly property color eyebrow: "#7f8c73"
    readonly property color em: "#56856b"
    readonly property color success: "#387657"
    readonly property color gold: "#e9bd68"

    property string serif: "serif"   // trocado pela Noto Serif do sistema no Main.qml

    function cardBg(c) {
        return ({ blue: "#deedf2", yellow: "#faedc8", green: "#e0eddc", purple: "#e8e2f1", peach: "#f6e7d5" })[c] || "#f7e1db"
    }
}
