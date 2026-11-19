import QtQuick

// Cartão de atividade da tela inicial.
Item {
    id: card
    property var game
    signal open()
    height: 290

    Rectangle { anchors.fill: body; anchors.topMargin: 4; anchors.bottomMargin: -4; radius: 22; color: "#e8e9de" }
    Rectangle {
        id: body
        width: parent.width; height: parent.height
        y: area.pressed ? 3 : 0
        radius: 22; color: "white"; border.color: "#e6e6d9"; clip: true
        Behavior on y { NumberAnimation { duration: 60 } }

        Rectangle {  // área da ilustração (cantos de cima arredondados)
            id: pic
            width: parent.width; height: 140; radius: 22
            color: Theme.cardBg(card.game.color)
            Rectangle { anchors.bottom: parent.bottom; width: parent.width; height: 22; color: parent.color }
            Art {
                anchors.centerIn: parent
                spec: card.game.art
                width: card.game.wide ? 180 : 110; height: card.game.wide ? 90 : 110
            }
            Text { text: "→"; rotation: -45; anchors.right: parent.right; anchors.top: parent.top; anchors.margins: 16; color: Theme.ink; font.pixelSize: 18 }
        }
        Column {
            anchors { top: pic.bottom; left: parent.left; right: parent.right; margins: 16; topMargin: 12 }
            spacing: 2
            Text { width: parent.width; text: card.game.name; font.family: Theme.serif; font.bold: true; font.pixelSize: 19; color: Theme.ink; wrapMode: Text.WordWrap }
            Text { width: parent.width; text: card.game.skill; font.pixelSize: 14; color: Theme.muted; wrapMode: Text.WordWrap }
            Item { width: 1; height: 8 }
            Rectangle {
                width: parent.width + 12; x: -6; height: prog.height + 12; radius: 9
                color: card.game.completed ? "#f1f7e9" : "transparent"
                Column {
                    id: prog; x: 6; y: 6; spacing: 3
                    Text {
                        text: card.game.completed ? "✓ Feita hoje" : card.game.solved > 0 ? "Em andamento hoje" : "Para descobrir hoje"
                        font.family: Theme.serif; font.bold: true; font.pixelSize: 14
                        color: card.game.completed ? Theme.success : Theme.ink
                    }
                    Text { text: card.game.solved + " de 5 descobertas"; font.pixelSize: 13; color: Theme.muted }
                    Row {
                        spacing: 6
                        Repeater {
                            model: 5
                            Rectangle {
                                width: 10; height: 10; radius: 5
                                color: index < card.game.solved ? Theme.green : "transparent"
                                border.width: index < card.game.solved ? 0 : 1.5; border.color: "#c9d3bf"
                            }
                        }
                    }
                }
            }
        }
    }
    MouseArea { id: area; anchors.fill: parent; onClicked: card.open() }
}
