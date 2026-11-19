import QtQuick

Row {
    spacing: 8
    Art { spec: "icon/leaf/-"; width: 34; height: 34; anchors.verticalCenter: parent.verticalCenter }
    Column {
        anchors.verticalCenter: parent.verticalCenter
        Text { text: "jardim"; font.family: Theme.serif; font.bold: true; font.pixelSize: 22; color: Theme.green }
        Text { text: "das descobertas"; font.pixelSize: 11; color: Theme.muted }
    }
}
