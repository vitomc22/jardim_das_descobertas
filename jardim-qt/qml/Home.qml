import QtQuick

// Tela inicial. Flickable = rolagem com inércia nativa; o grafo de cena só
// translada os nós já desenhados (sem redesenhar tudo a cada frame).
Flickable {
    id: home
    signal adults()
    contentHeight: col.height + 40
    boundsBehavior: Flickable.DragOverBounds
    flickDeceleration: 1500
    maximumFlickVelocity: 6000

    function toActivities() { contentY = Math.min(activities.y, contentHeight - height) }

    // ---- Reamostragem do toque por frame -----------------------------------
    // O Android entrega o toque na thread de UI do Java; o Qt desenha em outra
    // thread, com outro relógio. Sem reamostrar, ~20% dos frames do arrasto
    // saíam com a lista parada e o seguinte "pulava" o dobro (medido no tablet).
    // Aqui a posição é calculada A CADA FRAME: último ponto do dedo + velocidade
    // × tempo desde ele (no máx. 20 ms). Ao soltar, a velocidade real vai para
    // o flick() nativo, que cuida da inércia.
    interactive: false
    readonly property real maxY: Math.max(0, contentHeight - height)
    DragHandler {
        id: drag
        target: null
        xAxis.enabled: false
        property real y0
        property real c0
        property real lastY
        property double lastT
        property real vel
        onActiveChanged: {
            if (active) {
                home.cancelFlick()
                y0 = lastY = centroid.scenePosition.y; c0 = home.contentY; lastT = Date.now(); vel = 0
            } else {
                home.flick(0, centroid.velocity.y)
            }
        }
        onCentroidChanged: if (active) { lastY = centroid.scenePosition.y; lastT = Date.now(); vel = centroid.velocity.y }
    }
    FrameAnimation {
        running: drag.active
        onTriggered: {
            const dt = Math.min(Date.now() - drag.lastT, 20) / 1000
            const y = drag.lastY + drag.vel * dt
            home.contentY = Math.max(0, Math.min(home.maxY, drag.c0 - (y - drag.y0)))
        }
    }

    Column {
        id: col
        width: home.width
        spacing: 0

        Header { width: parent.width; onAdults: home.adults() }
        Item { width: 1; height: 18 }

        // Hero
        Item {
            width: parent.width
            height: Math.max(heroText.height, garden.height)
            Column {
                id: heroText
                width: parent.width - garden.width - 20
                anchors.verticalCenter: parent.verticalCenter
                spacing: 6
                Text { text: "BRINCAR • PENSAR • DESCOBRIR"; font.pixelSize: 12; font.letterSpacing: 2; font.bold: true; color: Theme.eyebrow }
                Text { text: "Pequenas descobertas."; font.family: Theme.serif; font.bold: true; font.pixelSize: 40; color: Theme.ink }
                Text { text: "Grandes sorrisos."; font.family: Theme.serif; font.bold: true; font.pixelSize: 40; color: Theme.em }
                Text { width: parent.width; text: "Um jardim de ideias para aprender brincando, um passo de cada vez."; wrapMode: Text.WordWrap; font.pixelSize: 17; color: Theme.muted }
                Item { width: 1; height: 10 }
                Btn { kind: "primary"; text: "Vamos explorar →"; onClicked: { app.explore(); home.toActivities() } }
            }
            Art { id: garden; spec: "garden"; anchors.right: parent.right; width: Math.min(460, home.width * 0.45); height: width * 300 / 520 }
        }
        Item { width: 1; height: 28 }

        // Faixas etárias
        Flow {
            id: activities
            width: parent.width
            spacing: 16
            Column {
                Text { text: "CADA IDADE, UMA DESCOBERTA"; font.pixelSize: 12; font.letterSpacing: 2; font.bold: true; color: Theme.eyebrow }
                Text { text: "O que vamos brincar hoje?"; font.family: Theme.serif; font.bold: true; font.pixelSize: 30; color: Theme.ink }
            }
            Row {
                spacing: 4
                Repeater {
                    model: app.groups
                    Btn { kind: "pill"; text: modelData.label; selected: modelData.id === app.group; onClicked: app.setGroup(modelData.id) }
                }
            }
        }
        Item { width: 1; height: 6 }
        Text { text: app.groupName + " • " + app.available + " atividades gratuitas"; font.pixelSize: 15; color: Theme.muted }
        Item { width: 1; height: 14 }

        // Progresso do dia
        Rectangle {
            width: parent.width; height: 92; radius: 22
            color: "#eaf0e2"; border.color: "#d4dfc7"
            Column {
                anchors { left: parent.left; leftMargin: 22; verticalCenter: parent.verticalCenter }
                Text { text: "SEU JARDIM DE HOJE"; font.pixelSize: 12; font.letterSpacing: 2; font.bold: true; color: Theme.eyebrow }
                Text { text: app.dailyDone + " de " + app.available + " atividades feitas hoje"; font.family: Theme.serif; font.bold: true; font.pixelSize: 21; color: Theme.ink }
                Text { text: "Novos desafios a cada dia. As marcações começam de novo diariamente."; font.pixelSize: 14; color: Theme.muted }
            }
            Row {
                id: meter
                anchors { right: parent.right; rightMargin: 22; verticalCenter: parent.verticalCenter }
                spacing: 10
                readonly property int pct: app.available ? Math.round(100 * app.dailyDone / app.available) : 0
                Rectangle {
                    width: 180; height: 12; radius: 6; color: "white"; anchors.verticalCenter: parent.verticalCenter
                    Rectangle { width: parent.width * meter.pct / 100; height: parent.height; radius: 6; color: Theme.green }
                }
                Text { text: meter.pct + "% hoje"; font.pixelSize: 15; color: Theme.ink }
            }
        }
        Item { width: 1; height: 16 }

        Flow {
            width: parent.width; spacing: 8
            Repeater {
                model: app.categories
                Btn { kind: "tab"; text: modelData.label; selected: modelData.id === app.category; onClicked: app.setCategory(modelData.id) }
            }
        }
        Item { width: 1; height: 16 }

        // Grade de cartões
        Grid {
            id: grid
            readonly property int cols: Math.max(1, Math.floor((home.width + 16) / (250 + 16)))
            columns: cols
            spacing: 16
            Repeater {
                model: app.cards
                GameCard {
                    width: (home.width - 16 * (grid.cols - 1)) / grid.cols
                    game: modelData
                    onOpen: app.start(modelData.id)
                }
            }
        }
        Item { width: 1; height: 20 }
        Rectangle { width: parent.width; height: 1; color: Theme.line }
        Item {
            width: parent.width; height: 48
            Text { anchors.verticalCenter: parent.verticalCenter; text: "Feito para a curiosidade florescer."; font.pixelSize: 14; color: Theme.muted }
            Text { anchors.verticalCenter: parent.verticalCenter; anchors.right: parent.right; text: "Gratuito · Offline · Sem anúncios"; font.pixelSize: 14; color: Theme.muted }
        }
    }
}
