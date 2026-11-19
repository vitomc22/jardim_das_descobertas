import QtQuick

Flickable {
    id: game
    readonly property var q: app.question
    readonly property var s: app.session
    contentHeight: col.height + 30
    boundsBehavior: Flickable.DragOverBounds

    Column {
        id: col
        width: game.width
        spacing: 12

        Item {
            width: parent.width; height: 56
            Row {
                anchors.verticalCenter: parent.verticalCenter; spacing: 16
                Btn { text: "← Jardim"; onClicked: app.home() }
                Logo { anchors.verticalCenter: parent.verticalCenter }
            }
            Btn { anchors.right: parent.right; anchors.verticalCenter: parent.verticalCenter
                text: app.voice ? "♪ Som ligado" : "♪ Som desligado"; onClicked: app.toggleVoice() }
        }

        Rectangle {
            width: parent.width; height: inner.height + 40
            radius: game.q.radius + 8; color: game.q.surface
            Behavior on color { ColorAnimation { duration: 250 } }

            Column {
                id: inner
                x: 20; y: 20; width: parent.width - 40; spacing: 10

                Item { width: parent.width; height: 22
                    Text { text: game.q.gameName; font.pixelSize: 15; color: Theme.muted }
                    Text { anchors.right: parent.right; text: "Descoberta " + (game.s.round + 1) + " de " + game.s.length; font.pixelSize: 15; color: Theme.muted } }

                Row {  // pontinhos da rodada
                    anchors.horizontalCenter: parent.horizontalCenter; spacing: 10
                    Repeater { model: game.s.length
                        Rectangle { width: index === game.s.round ? 14 : 12; height: width; radius: width / 2; anchors.verticalCenter: parent.verticalCenter
                            color: index < game.s.round ? Theme.green : index === game.s.round ? Theme.gold : "transparent"
                            border.width: index > game.s.round ? 1.5 : 0; border.color: "#c9d3bf" } } }

                Row {
                    anchors.horizontalCenter: parent.horizontalCenter; spacing: 12
                    Text { id: prompt; text: game.q.prompt; font.family: Theme.serif; font.bold: true; font.pixelSize: 30; color: Theme.ink
                        width: Math.min(implicitWidth, inner.width - 80); wrapMode: Text.WordWrap; anchors.verticalCenter: parent.verticalCenter }
                    Rectangle { width: 52; height: 52; radius: 26; color: "white"; border.color: Theme.line; anchors.verticalCenter: parent.verticalCenter
                        Text { anchors.centerIn: parent; text: "🔊"; font.pixelSize: 22; color: Theme.green }
                        MouseArea { anchors.fill: parent; onClicked: app.listen() } }
                }
                Text { anchors.horizontalCenter: parent.horizontalCenter; font.pixelSize: 15; color: Theme.muted
                    text: game.q.memory ? "Toque nas cartas para descobrir os desenhos." : "Toque na resposta que você escolheu." }

                SceneView { width: parent.width; visible: !game.q.memory && game.q.hasScene; scene: game.q.scene; q: game.q; counted: game.s.counted }

                // Respostas
                Flow {
                    visible: !game.q.memory
                    anchors.horizontalCenter: parent.horizontalCenter
                    width: Math.min(parent.width, (game.q.options ? game.q.options.length : 0) * 168)
                    spacing: 18
                    Repeater {
                        model: game.q.options
                        Item {
                            id: choice
                            width: 150; height: 146
                            readonly property bool isRight: game.s.correct && index === game.q.answer
                            readonly property bool retry: game.s.wiggle === index
                            Rectangle { anchors.fill: face; anchors.topMargin: 6; anchors.bottomMargin: -6; radius: face.radius
                                color: choice.isRight ? "#78a36d" : "#e3e6d6" }
                            Rectangle {
                                id: face
                                width: 150; height: 140
                                y: area.pressed ? 3 : 0
                                radius: Math.min(game.q.radius, 40)
                                color: choice.isRight ? "#e1eed8" : choice.retry ? "#f8ebdb" : "white"
                                border.width: 2; border.color: choice.isRight ? "#78a36d" : "#e5e7d9"
                                Behavior on color { ColorAnimation { duration: 150 } }
                                Visual { anchors.fill: parent; anchors.margins: 13; visual: modelData.visual; rotations: game.q.rotation; itemColor: game.q.itemColor }
                                SequentialAnimation on x {  // "wiggle" ao errar
                                    id: wiggle; running: false
                                    NumberAnimation { to: -6; duration: 60 } NumberAnimation { to: 6; duration: 80 }
                                    NumberAnimation { to: -4; duration: 80 } NumberAnimation { to: 4; duration: 80 } NumberAnimation { to: 0; duration: 60 }
                                }
                                Connections { target: app; function onChanged() { if (choice.retry && game.s.wiggleSerial !== choice.lastSerial) { choice.lastSerial = game.s.wiggleSerial; wiggle.restart() } } }
                            }
                            property int lastSerial: -1
                            MouseArea { id: area; anchors.fill: parent; enabled: !game.s.correct; onClicked: app.answer(index) }
                        }
                    }
                }

                // Memória
                Grid {
                    visible: game.q.memory
                    anchors.horizontalCenter: parent.horizontalCenter
                    columns: game.q.cards && game.q.cards.length <= 4 ? 2 : 4
                    spacing: 16
                    Repeater {
                        model: game.q.memory ? game.q.cards : []
                        Item {
                            id: mc
                            width: 140; height: 145
                            readonly property bool matched: game.s.matched.indexOf(index) >= 0
                            readonly property bool open: matched || game.s.revealed.indexOf(index) >= 0
                            Rectangle { anchors.fill: cardFace; anchors.topMargin: 5; anchors.bottomMargin: -5; radius: 20; color: mc.matched ? "#95b681" : "#c3d2b4" }
                            Rectangle {
                                id: cardFace
                                width: 140; height: 140; radius: 20
                                color: mc.matched ? "#eff5e9" : mc.open ? "#fffdf7" : "#e2ead6"
                                border.width: 2; border.color: mc.matched ? "#95b681" : mc.open ? "#d8cba5" : "#c5d6b7"
                                Art { anchors.fill: parent; anchors.margins: 22; spec: mc.open ? "icon/" + modelData.kind + "/-" : "icon/leaf/-" }
                            }
                            MouseArea { anchors.fill: parent; enabled: !mc.matched; onClicked: app.flip(index) }
                        }
                    }
                }

                Rectangle { width: parent.width; height: 1; color: Theme.line }
                Item {
                    width: parent.width; height: 60
                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        width: parent.width - 280; wrapMode: Text.WordWrap
                        readonly property string fb: game.s.feedback
                        text: fb === "success" ? "★ Você descobriu! Muito bem." : fb === "hint" ? game.q.hint
                            : fb === "retry" ? "Vamos tentar de novo? " + game.q.hint : "Vamos descobrir juntos?"
                        color: fb === "success" ? Theme.success : Theme.ink
                        font.family: fb === "success" ? Theme.serif : Qt.application.font.family; font.bold: fb === "success"
                        font.pixelSize: fb === "success" ? 18 : 17
                    }
                    Btn {
                        anchors.right: parent.right; anchors.verticalCenter: parent.verticalCenter
                        kind: game.s.correct ? "primary" : "quiet"
                        text: game.s.correct ? (game.s.round + 1 === game.s.length ? "Ver meu jardim →" : "Próxima descoberta →")
                            : game.s.feedback === "idle" ? "Uma dica" : "Ouvir dica"
                        onClicked: game.s.correct ? app.next() : app.hint()
                    }
                }
            }
        }
    }
}
