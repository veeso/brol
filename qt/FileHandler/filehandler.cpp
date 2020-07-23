#include "filehandler.h"

/**
 * @brief FileHandler::writeFile - write a file with a certain content
 * @param filePath - path of the file on filesystem
 * @param content - content to write to file
 * @param mode - 'a' append, 'w' write
 * @return
 */


bool FileHandler::writeFile(QString filePath, QString content, QChar mode) {

    QFile thisFile(filePath);

    char m = mode.toLatin1();

    switch(m) {

    case 'w':
    case 'W':
        if(!thisFile.open(QIODevice::WriteOnly)) {
            qDebug() << "FileHandler::writeFile Could not open file " << filePath;
            return false;
        }
        break;

    case 'a':
    case 'A':
        if(!thisFile.open(QIODevice::Append)) {
            qDebug() << "FileHandler::writeFile Could not open file " << filePath;
            return false;
        }
        break;

    default:
        qDebug() << "Unknown mode " << m;
        return false;
    }

    QTextStream dataOut(&thisFile);
    dataOut << content;

    thisFile.close();

    return true;



}

/**
 * @brief FileHandler::readFile - read the content of a file
 * @param filePath
 * @return
 */

QString FileHandler::readFile(QString filePath) {

    QFile thisFile(filePath);
    if(!thisFile.exists()) {
        qDebug() << "FileHandler::readFile File does not exist: " << filePath;
        return "";
    }
    if(!thisFile.open(QIODevice::ReadOnly)) {
        qDebug() << "FileHandler::readFile Could not open file " << filePath;
        return "";
    }

    QString content = thisFile.readAll();
    thisFile.close();

    return content;

}

/**
 * @brief FileHandler::deleteFile - Delete a file on the filesystem
 * @param filePath
 * @return bool
 */

bool FileHandler::deleteFile(QString filePath) {

    QFile thisFile(filePath);
    if(!thisFile.exists()) {
        qDebug() << "FileHandler::deleteFile File does not exist: " << filePath;
        return 1;
    }

    return thisFile.remove();

}
