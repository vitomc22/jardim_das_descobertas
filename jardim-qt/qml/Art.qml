import QtQuick
import QtQuick.Window

// Desenho SVG rasterizado em C++ (thread pool) no tamanho exato em pixels
// físicos e guardado no cache de texturas do Qt Quick.
Image {
    property string spec
    source: spec ? "image://svg/" + spec : ""
    sourceSize: Qt.size(Math.ceil(width * Screen.devicePixelRatio), Math.ceil(height * Screen.devicePixelRatio))
    asynchronous: true
    cache: true
    smooth: true
    fillMode: Image.PreserveAspectFit
}
