#ifndef PROCESS_H
#define PROCESS_H

#include <QProcess>
#include <QVariant>

class Process : public QProcess {
    Q_OBJECT

public:
    Process(QObject *parent = 0) : QProcess(parent) { }
    Q_INVOKABLE int start(const QString &program, const QVariantList &arguments);
    Q_INVOKABLE QString startAndRead(const QString &program, const QVariantList &arguments);
    Q_INVOKABLE QByteArray readAll();

};

#endif
