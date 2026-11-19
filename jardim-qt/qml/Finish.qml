import QtQuick

Item {
    id: fin
    signal adults()
    readonly property var s: app.session
    Header { id: hdr; width: parent.width; onAdults: fin.adults() }
    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.top: hdr.bottom; anchors.topMargin: 30
        spacing: 6
        Art { spec: "icon/flower/-"; width: 130; height: 130; anchors.horizontalCenter: parent.horizontalCenter
            NumberAnimation on scale { from: 0.6; to: 1; duration: 450; easing.type: Easing.OutBack } }
        Text { anchors.horizontalCenter: parent.horizontalCenter; text: "A CURIOSIDADE FLORESCEU!"; font.pixelSize: 12; font.letterSpacing: 2; font.bold: true; color: Theme.eyebrow }
        Text { anchors.horizontalCenter: parent.horizontalCenter; font.family: Theme.serif; font.bold: true; font.pixelSize: 40; color: Theme.ink
            text: fin.s.length === 1 ? "Uma nova descoberta!" : fin.s.length + " novas descobertas!" }
        Text { anchors.horizontalCenter: parent.horizontalCenter; font.family: Theme.serif; font.bold: true; font.pixelSize: 18; color: Theme.success
            text: fin.s.dailyCompleted ? "✓ Atividade feita hoje!" : fin.s.dailySolved + " de 5 descobertas hoje" }
        Item { width: 1; height: 6 }
        Text { anchors.horizontalCenter: parent.horizontalCenter; text: "Você observou, tentou e aprendeu."; font.pixelSize: 17; color: Theme.muted }
        Text { anchors.horizontalCenter: parent.horizontalCenter; text: "Que tal alongar o corpo e olhar o mundo lá fora?"; font.pixelSize: 17; color: Theme.muted }
        Text { anchors.horizontalCenter: parent.horizontalCenter; text: "★ ".repeat(fin.s.length); font.pixelSize: 22; color: Theme.gold }
        Item { width: 1; height: 10 }
        Btn { anchors.horizontalCenter: parent.horizontalCenter; kind: "primary"; text: "Voltar para o jardim"; onClicked: app.home() }
        Item { width: 1; height: 6 }
        Text { anchors.horizontalCenter: parent.horizontalCenter; text: "Cada tentativa faz parte de aprender."; font.pixelSize: 14; color: Theme.muted }
    }
}
