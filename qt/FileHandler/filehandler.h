#ifndef FILEHANDLER_H
#define FILEHANDLER_H

#include <QObject>
#include <QString>
#include <QVariant>
#include <QFile>
#include <QList>
#include <QTextStream>
#include <QtDebug>


class FileHandler : public QObject
{
    Q_OBJECT

public:
    FileHandler(QObject *parent = 0) : QObject(parent) {
    }
    Q_INVOKABLE bool writeFile(QString filePath, QString content, QChar mode);
    Q_INVOKABLE QString readFile(QString filePath);
    Q_INVOKABLE bool deleteFile(QString filePath);
};

#endif // FILEHANDLER_H
