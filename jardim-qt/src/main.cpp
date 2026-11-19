#include "app.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>

int main(int argc, char* argv[]) {
#ifdef JARDIM_TRACE
    qputenv("QSG_RENDER_TIMING", "1");
#endif
    QGuiApplication a(argc, argv);
    QGuiApplication::setApplicationName("Jardim das Descobertas");

    App app;
    QQmlApplicationEngine engine;
    engine.addImageProvider("svg", new SvgProvider);
    engine.rootContext()->setContextProperty("app", &app);
#ifdef JARDIM_TRACE
    engine.rootContext()->setContextProperty("jardimTrace", true);
#else
    engine.rootContext()->setContextProperty("jardimTrace", false);
#endif

    QObject::connect(&a, &QGuiApplication::applicationStateChanged, &app, [&app](Qt::ApplicationState s) {
        if (s == Qt::ApplicationActive) app.resume(); else app.stopVoice();
    });
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreationFailed, &a, [] { QCoreApplication::exit(1); }, Qt::QueuedConnection);
    engine.loadFromModule("Jardim", "Main");
    return a.exec();
}
