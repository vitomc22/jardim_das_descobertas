import QtQuick
import QtQuick.Window

Window {
    id: win
    visible: true
    width: 1280; height: 800
    color: Theme.paper
    title: "Jardim das Descobertas"

    FontLoader { id: serifBold; source: "file:///system/fonts/NotoSerif-Bold.ttf" }
    Binding { target: Theme; property: "serif"; value: serifBold.status === FontLoader.Ready ? serifBold.font.family : "serif" }

    property string modal: ""      // "", "gate", "parents", "reset"
    property int gateA: 6
    property int gateB: 6
    property string gateInput: ""
    property bool gateError: false

    function openGate() {
        app.stopVoice()
        gateA = 6 + Math.floor(Math.random() * 4); gateB = 6 + Math.floor(Math.random() * 4)
        gateInput = ""; gateError = false; modal = "gate"
    }
    // Voltar do Android: fecha diálogo → volta ao jardim → sai.
    function back() {
        if (modal !== "") { modal = ""; return true }
        if (app.screen !== "home") { app.home(); return true }
        return false
    }
    onClosing: (close) => { if (back()) close.accepted = false }

    // Diagnóstico: posição da lista a cada frame (thread de GUI).
    onAfterAnimating: if (jardimTrace && page.item && page.item.contentY !== undefined)
        console.log("JT " + Date.now() + " " + page.item.contentY.toFixed(1) + " " + (page.item.dragging ? 1 : 0) + " " + (page.item.flicking ? 1 : 0))

    Item {
        id: safe
        anchors.fill: parent
        focus: true
        Keys.onBackPressed: (e) => { e.accepted = win.back() }
        // Barras do sistema (Android 15 é edge-to-edge).
        anchors.topMargin: win.contentItem.SafeArea.margins.top + 16
        anchors.bottomMargin: win.contentItem.SafeArea.margins.bottom
        anchors.leftMargin: win.contentItem.SafeArea.margins.left + 24
        anchors.rightMargin: win.contentItem.SafeArea.margins.right + 24

        Loader {
            id: page
            anchors.fill: parent
            clip: true
            sourceComponent: app.screen === "game" ? gameC : app.screen === "finish" ? finishC : homeC
        }
        Component { id: homeC; Home { onAdults: win.openGate() } }
        Component { id: gameC; Game {} }
        Component { id: finishC; Finish { onAdults: win.openGate() } }
    }

    // ------------------------------------------------------------ diálogos
    Rectangle {  // fundo escurecido simples (sem blur)
        anchors.fill: parent
        visible: win.modal !== ""
        color: "#a8233b36"
        MouseArea { anchors.fill: parent }
        Rectangle {
            anchors.centerIn: parent
            width: Math.min(parent.width - 40, 560)
            height: Math.min(dlg.height + 60, parent.height * 0.9)
            radius: 27; color: Theme.paper
            Flickable {
                anchors.fill: parent; anchors.margins: 30
                contentHeight: dlg.height; clip: true
                Loader {
                    id: dlg
                    width: parent.width
                    sourceComponent: win.modal === "gate" ? gateC : win.modal === "parents" ? parentsC : win.modal === "reset" ? resetC : null
                }
            }
        }
    }

    component Eyebrow: Text { font.pixelSize: 12; font.letterSpacing: 2; font.bold: true; color: Theme.eyebrow }
    component H2: Text { font.family: Theme.serif; font.bold: true; font.pixelSize: 26; color: Theme.ink; width: parent.width; wrapMode: Text.WordWrap }
    component H3: Text { font.family: Theme.serif; font.bold: true; font.pixelSize: 18; color: Theme.ink }
    component P: Text { font.pixelSize: 15; color: Theme.muted; width: parent.width; wrapMode: Text.WordWrap }

    Component {
        id: gateC
        Column {
            spacing: 10
            Eyebrow { text: "UM MOMENTO PARA OS ADULTOS" }
            H2 { text: "Vamos cuidar do jardim?" }
            P { text: "Para abrir as configurações, resolva:" }
            Row {
                spacing: 12
                Text { text: "Quanto é " + win.gateA + " × " + win.gateB + "?"; font.family: Theme.serif; font.bold: true; font.pixelSize: 22; color: Theme.ink; anchors.verticalCenter: parent.verticalCenter }
                Rectangle { width: 110; height: 50; radius: 12; color: "white"; border.width: 2; border.color: Theme.line
                    Text { anchors.centerIn: parent; text: win.gateInput; font.family: Theme.serif; font.bold: true; font.pixelSize: 26; color: Theme.ink } }
            }
            Text { visible: win.gateError; text: "Tente mais uma vez."; color: "#9c513c"; font.pixelSize: 15 }
            Grid {  // teclado numérico próprio
                columns: 3; spacing: 8
                Repeater {
                    model: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "⌫", "0"]
                    Rectangle {
                        width: 72; height: 52; radius: 12; color: ka.pressed ? Theme.line : Theme.quiet
                        Text { anchors.centerIn: parent; text: modelData; font.family: Theme.serif; font.bold: true; font.pixelSize: 22; color: Theme.ink }
                        MouseArea { id: ka; anchors.fill: parent; onClicked: {
                            win.gateError = false
                            if (modelData === "⌫") win.gateInput = win.gateInput.slice(0, -1)
                            else if (win.gateInput.length < 3) win.gateInput += modelData } }
                    }
                }
            }
            Row {
                spacing: 10
                Btn { text: "Voltar"; onClicked: win.modal = "" }
                Btn { kind: "primary"; text: "Entrar"; onClicked: {
                    if (parseInt(win.gateInput) === win.gateA * win.gateB) win.modal = "parents"; else win.gateError = true } }
            }
        }
    }

    Component {
        id: parentsC
        Column {
            spacing: 10
            Eyebrow { text: "ÁREA DOS ADULTOS" }
            H2 { text: "Aprender no próprio ritmo" }
            P { text: "As faixas são sugestões. Escolha a que combina com a criança e acompanhe as atividades, especialmente aos 2–3 anos." }
            H3 { text: "Faixa etária" }
            Row { spacing: 4; Repeater { model: app.groups; Btn { kind: "pill"; text: modelData.label; selected: modelData.id === app.group; onClicked: app.setGroup(modelData.id) } } }
            H3 { text: "Narração em português" }
            Btn { text: app.voice ? "Ligada — toque para desligar" : "Desligada — toque para ligar"; onClicked: app.toggleVoice() }
            P { font.pixelSize: 13; text: "Usa a voz do Android. O primeiro uso pode exigir uma voz pt-BR já instalada. As atividades funcionam sem áudio." }
            H3 { text: "Descobertas por atividade · " + app.groups.find(g => g.id === app.group).label }
            Repeater {
                model: app.parentStats
                Item { width: parent.width; height: 22
                    Text { text: modelData.name; font.pixelSize: 14; color: Theme.ink }
                    Text { anchors.right: parent.right; text: modelData.today + "/5 hoje · " + modelData.total + " no total"; font.pixelSize: 14; color: Theme.muted } }
            }
            P { font.pixelSize: 13; text: "Os dados ficam somente neste dispositivo. Sem conta, anúncios, compras ou acesso à internet. A pergunta de entrada é uma barreira simples para crianças pequenas." }
            Row { spacing: 10
                Btn { kind: "danger"; text: "Apagar progresso"; onClicked: win.modal = "reset" }
                Btn { kind: "primary"; text: "Pronto"; onClicked: { win.modal = ""; app.home() } } }
        }
    }

    Component {
        id: resetC
        Column {
            spacing: 12
            H2 { text: "Apagar todas as descobertas?" }
            P { text: "Isso apaga o progresso das três faixas etárias neste dispositivo." }
            Row { spacing: 10
                Btn { text: "Cancelar"; onClicked: win.modal = "parents" }
                Btn { kind: "danger"; text: "Sim, apagar"; onClicked: { app.reset(); win.modal = "parents" } } }
        }
    }
}
