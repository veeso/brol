#include "process.h"
#include <QFileInfo>
#include <QtDebug>

int Process::start(const QString &program, const QVariantList &arguments) {
    QStringList args;

    // convert QVariantList from QML to QStringList for QProcess

    for (int i = 0; i < arguments.length(); i++) {
        args << arguments[i].toString();
    }

    bool pathIsRelative = true;

    //Check if program is an absolute path
    if(program.at(0) == '/') {
        pathIsRelative = false;
    }

    //Check if file exists OR path is relative: if exists, start program and wait it for finishing before emitting the signal
    QFileInfo check_file(program);
    if((check_file.exists() && check_file.isFile()) || pathIsRelative) {
        //R1.2.3 - Before trying to start new process, wait for the older to finish (added because aplay tried to play even if another aplay was playing a sound, preventing the newest sound to be played)
        QProcess::start(program, args);
        QProcess::waitForFinished(15000);
        emit finished(QProcess::exitCode(), QProcess::NormalExit);
        return QProcess::exitCode();
        //qDebug() << "Process " << program << args << " started";
    }
    //If not emit immediately processFinished
    else {
        emit finished(-1, QProcess::NormalExit);
        qDebug() << program << ": no such file or directory!";
        return 1;
    }

}

QString Process::startAndRead(const QString &program, const QVariantList &arguments) {
    QStringList args;

    // convert QVariantList from QML to QStringList for QProcess

    for (int i = 0; i < arguments.length(); i++) {
        args << arguments[i].toString();
    }

    bool pathIsRelative = true;

    //Check if program is an absolute path
    if(program.at(0) == '/') {
        pathIsRelative = false;
    }

    //Check if file exists OR path is relative: if exists, start program and wait it for finishing before emitting the signal
    QFileInfo check_file(program);
    if((check_file.exists() && check_file.isFile()) || pathIsRelative) {
        //R1.2.3 - Before trying to start new process, wait for the older to finish (added because aplay tried to play even if another aplay was playing a sound, preventing the newest sound to be played)
        QProcess::start(program, args);
        QProcess::waitForFinished(15000);
        QString stdoutByte(QProcess::readAllStandardOutput());
        return stdoutByte;
    }
    //If not emit immediately processFinished
    else {
        emit finished(-1, QProcess::NormalExit);
        qDebug() << program << ": no such file or directory!";
        return "";
    }
}

QByteArray Process::readAll() {
    QByteArray stdoutByte = QProcess::readAllStandardOutput();
    //qDebug() << "Process output: " << stdoutByte;
    return stdoutByte;
}

