import QtQuick

// Conteúdo de uma opção de resposta (equivale a visual() do app.js).
Item {
    id: v
    property var visual
    property var rotations: []
    property color itemColor: "#e87961"

    Art {
        visible: !!v.visual.art
        anchors.centerIn: parent
        spec: v.visual.art || ""
        readonly property real s: Math.min(v.width, v.height) * (v.visual.type === "sized" ? v.visual.number : 1)
        width: s; height: s
    }
    Text {
        visible: v.visual.type === "number" || v.visual.type === "text"
        anchors.centerIn: parent
        text: v.visual.type === "number" ? v.visual.number : v.visual.text
        font.family: Theme.serif; font.bold: true
        font.pixelSize: v.visual.type === "text" && v.visual.text.length > 3 ? 28 : v.visual.type === "number" ? 52 : 46
        color: Theme.ink
    }
    Rectangle {
        visible: v.visual.type === "swatch"
        anchors.centerIn: parent; width: 80; height: 80; radius: 18
        color: v.visual.color || "transparent"
    }
    Grid {  // grupo de maçãs
        id: qgrid
        visible: v.visual.type === "quantity"
        anchors.centerIn: parent
        readonly property int n: v.visual.type === "quantity" ? v.visual.number : 0
        columns: Math.max(1, Math.min(n, n <= 4 ? 2 : n <= 9 ? 3 : 4))
        readonly property real cell: Math.min(v.width / columns, v.height / Math.ceil(n / columns))
        Repeater {
            model: parent.n
            Art {
                width: qgrid.cell; height: qgrid.cell
                spec: "icon/apple/" + String(v.itemColor).slice(1)
                rotation: v.rotations[index % 24] || 0
            }
        }
    }
}
