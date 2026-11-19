import QtQuick

// Botões do app: "quiet" (pílula cinza), "primary" (verde com sombra),
// "pill" (faixa etária), "tab" (categoria), "danger".
Item {
    id: b
    property string text
    property string kind: "quiet"
    property bool selected: false
    property real fontSize: kind === "primary" ? 18 : 15
    signal clicked()

    implicitWidth: label.implicitWidth + (kind === "primary" ? 48 : 34)
    implicitHeight: kind === "primary" ? 56 : kind === "tab" ? 42 : 46

    readonly property color fill: kind === "primary" ? Theme.green
        : kind === "danger" ? "#f2e2d8"
        : kind === "pill" ? (selected ? Theme.green : "transparent")
        : kind === "tab" ? (selected ? "#e0eada" : "transparent")
        : (area.pressed ? Theme.line : Theme.quiet)
    readonly property color fg: kind === "primary" ? "white"
        : kind === "danger" ? "#9c513c"
        : kind === "pill" ? (selected ? "white" : Theme.muted)
        : kind === "tab" ? (selected ? "#346950" : Theme.muted)
        : Theme.ink

    Rectangle {  // sombra sólida (box-shadow 0 4px 0)
        visible: b.kind === "primary"
        anchors.fill: face; anchors.topMargin: 4; anchors.bottomMargin: -4
        radius: face.radius; color: Theme.greenShadow
    }
    Rectangle {
        id: face
        width: parent.width; height: parent.height
        y: area.pressed && b.kind === "primary" ? 3 : 0
        radius: b.kind === "primary" ? 16 : b.kind === "danger" ? 15 : height / 2
        color: b.fill
        border.width: b.kind === "quiet" || b.kind === "tab" ? 1 : 0
        border.color: b.kind === "tab" ? (b.selected ? "#9dba99" : Theme.line) : Theme.quietBorder
        Behavior on y { NumberAnimation { duration: 60 } }
        Text {
            id: label
            anchors.centerIn: parent
            text: b.text
            color: b.fg
            font.pixelSize: b.fontSize
            font.family: b.kind === "primary" || b.kind === "danger" ? Theme.serif : Qt.application.font.family
            font.bold: b.kind === "primary" || b.kind === "danger"
        }
    }
    MouseArea { id: area; anchors.fill: parent; onClicked: b.clicked() }
}
