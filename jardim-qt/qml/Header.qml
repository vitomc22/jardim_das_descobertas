import QtQuick

Item {
    id: hdr
    signal adults()
    height: 56
    Logo { anchors.left: parent.left; anchors.verticalCenter: parent.verticalCenter }
    Row {
        anchors.right: parent.right; anchors.verticalCenter: parent.verticalCenter
        spacing: 12
        Text { text: "★ " + app.total + " descobertas"; color: Theme.eyebrow; font.pixelSize: 15; anchors.verticalCenter: parent.verticalCenter }
        Btn { text: "Área dos adultos"; onClicked: hdr.adults() }
    }
}
