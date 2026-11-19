import QtQuick

// A "cena" acima das respostas (equivale a scene()/sceneExtra() do app.js).
Item {
    id: sv
    property var scene
    property var q
    property var counted: []
    readonly property string t: scene ? scene.type : ""
    implicitHeight: content.item ? content.item.height : 0

    function hexNo(c) { return String(c).slice(1) }

    component Missing: Rectangle {
        radius: 14; color: "#fffdf8"; border.width: 2.5; border.color: "#e9bd68"
        Text { anchors.centerIn: parent; text: "?"; font.family: Theme.serif; font.bold: true; font.pixelSize: 34; color: "#c8963c" }
    }
    component Apple: Item {
        property int idx
        property real size: 64
        width: size; height: size
        readonly property bool on: sv.counted.indexOf(idx) >= 0
        Rectangle { anchors.fill: parent; radius: width / 2; color: "#e0eddc"; border.width: 2; border.color: "#78a36d"; visible: parent.on }
        Art { anchors.fill: parent; anchors.margins: 4; spec: "icon/apple/" + sv.hexNo(sv.q.itemColor); rotation: sv.q.rotation[idx % 24] }
        MouseArea { anchors.fill: parent; onClicked: app.countItem(parent.idx) }
    }

    Loader {
        id: content
        width: sv.width
        sourceComponent: ({
            "color": colorC, "shape": artC, "sock": artC, "tower": artC, "clock": artC, "object-model": modelC,
            "items": itemsC, "sum": sumC, "look": lookC, "sequence": seqC, "numbers": numbersC, "friends": friendsC,
            "color-sequence": colorSeqC, "letter": letterC, "word-picture": wordC, "subtract": subC, "equation": eqC,
            "tenframe": tenC, "tens": tensC, "story": storyC })[sv.t] || null
    }

    Component { id: colorC; Item { height: 120; Rectangle { anchors.centerIn: parent; width: 120; height: 120; radius: 60; color: sv.scene.color } } }
    Component { id: artC; Item { height: sv.t === "clock" ? 170 : 150; Art { anchors.centerIn: parent; width: parent.height; height: parent.height; spec: sv.scene.art } } }
    Component { id: modelC; Item { height: 150; Rectangle { anchors.centerIn: parent; width: 150; height: 150; radius: 24; color: "white"
        Art { anchors.fill: parent; anchors.margins: 14; spec: "icon/" + sv.scene.icon + "/-" } } } }
    Component { id: lookC; Item { height: 28; Text { anchors.centerIn: parent; text: "Observe com atenção. Você consegue!"; font.pixelSize: 16; color: Theme.muted } } }
    Component { id: itemsC; Item { height: flow.height
        Flow { id: flow; width: Math.min(parent.width, sv.scene.count * 72); anchors.horizontalCenter: parent.horizontalCenter; spacing: sv.q.itemGap
            Repeater { model: sv.scene.count; Apple { idx: index } } } } }
    Component { id: sumC; Item { height: row.height
        Row { id: row; anchors.horizontalCenter: parent.horizontalCenter; spacing: 16
            Flow { width: Math.min(sv.scene.a, 5) * 52; spacing: 6; anchors.verticalCenter: parent.verticalCenter
                Repeater { model: sv.scene.a; Apple { idx: index; size: 46 } } }
            Text { text: "+"; font.family: Theme.serif; font.bold: true; font.pixelSize: 40; color: Theme.ink; anchors.verticalCenter: parent.verticalCenter }
            Flow { width: Math.min(sv.scene.b, 5) * 52; spacing: 6; anchors.verticalCenter: parent.verticalCenter
                Repeater { model: sv.scene.b; Apple { idx: sv.scene.a + index; size: 46 } } } } } }
    Component { id: seqC; Item { height: 84
        Row { anchors.centerIn: parent; spacing: 6
            Repeater { model: sv.scene.items; Art { width: 84; height: 84; spec: "icon/" + modelData + "/8b7bbd" } }
            Missing { width: 72; height: 72; anchors.verticalCenter: parent.verticalCenter } } } }
    Component { id: numbersC; Item { height: 120
        Row { anchors.centerIn: parent; spacing: 12
            Repeater { model: sv.scene.numbers
                Item { width: 80; height: 120
                    Rectangle { visible: modelData !== undefined && modelData !== null; width: 80; height: 70; y: 50 - index * 12; radius: 14; color: "white"; border.width: 2; border.color: Theme.line
                        Text { anchors.centerIn: parent; text: modelData === null || modelData === undefined ? "" : modelData; font.family: Theme.serif; font.bold: true; font.pixelSize: 34; color: Theme.ink } }
                    Missing { visible: modelData === null || modelData === undefined; width: 80; height: 70; y: 50 - index * 12 } } } } } }
    Component { id: friendsC; Item { height: 80
        Row { anchors.centerIn: parent; Repeater { model: sv.scene.count; Art { width: 80; height: 78; spec: "icon/bunny/-" } } } } }
    Component { id: colorSeqC; Item { height: 76
        Row { anchors.centerIn: parent; spacing: 16
            Repeater { model: sv.scene.items; Rectangle { width: 68; height: 68; radius: 34; color: modelData } }
            Missing { width: 68; height: 68 } } } }
    Component { id: letterC; Item { height: 120; Rectangle { anchors.centerIn: parent; width: 260; height: 120; radius: 24; color: "white"
        Text { anchors.centerIn: parent; text: sv.scene.text; font.family: Theme.serif; font.bold: true; font.pixelSize: 64; color: Theme.ink } } } }
    Component { id: wordC; Item { height: 130
        Row { anchors.centerIn: parent; spacing: 20
            Art { width: 130; height: 130; spec: "icon/" + sv.scene.icon + "/-" }
            Text { visible: sv.scene.word !== ""; text: sv.scene.word; font.family: Theme.serif; font.bold: true; font.pixelSize: 46; color: Theme.ink; anchors.verticalCenter: parent.verticalCenter } } } }
    Component { id: subC; Item { height: 52
        Row { anchors.centerIn: parent; spacing: 6
            Repeater { model: sv.scene.a
                Item { width: 52; height: 52
                    readonly property bool gone: index >= sv.scene.a - sv.scene.b
                    Art { anchors.fill: parent; spec: "icon/apple/-"; opacity: parent.gone ? 0.4 : 1 }
                    Rectangle { visible: parent.gone; anchors.centerIn: parent; width: 58; height: 4; radius: 2; rotation: 45; color: "#c95a46" } } } } } }
    Component { id: eqC; Item { height: 76
        Row { anchors.centerIn: parent; spacing: 8
            Repeater { model: [String(sv.scene.a), "+", null, "=", String(sv.scene.count)]
                Item { width: 72; height: 72
                    Text { visible: modelData !== null; anchors.centerIn: parent; text: modelData || ""; font.family: Theme.serif; font.bold: true; font.pixelSize: 40; color: Theme.ink }
                    Missing { visible: modelData === null; anchors.fill: parent } } } } } }
    Component { id: tenC; Item { height: 124
        Rectangle { anchors.centerIn: parent; width: 5 * 56 + 12; height: 2 * 56 + 12; radius: 12; color: "white"; border.width: 2; border.color: "#92b4ad"
            Grid { x: 6; y: 6; columns: 5
                Repeater { model: 10
                    Rectangle { width: 56; height: 56; color: "transparent"; border.color: Theme.line
                        Rectangle { visible: index < sv.scene.count; anchors.centerIn: parent; width: 38; height: 38; radius: 19; color: "#e87961" } } } } } } }
    Component { id: tensC; Item { height: 134
        Row { anchors.centerIn: parent; spacing: 30
            Row { spacing: 14
                Repeater { model: sv.scene.a; Column { Repeater { model: 10; Rectangle { width: 13; height: 13; radius: 2; color: "#5799be"; border.width: 0.5; border.color: "white" } } } } }
            Row { spacing: 4; anchors.bottom: parent.bottom
                Repeater { model: sv.scene.b; Rectangle { width: 13; height: 13; radius: 2; color: "#e0b849" } } } } } }
    Component { id: storyC; Item { height: Math.max(96, storyText.height)
        Row { anchors.horizontalCenter: parent.horizontalCenter; spacing: 12
            Art { width: 96; height: 96; spec: "icon/" + sv.scene.icon + "/-" }
            Text { id: storyText; width: Math.min(sv.width, 720) - 110; text: sv.scene.text; wrapMode: Text.WordWrap; font.pixelSize: 19; color: Theme.ink } } } }
}
