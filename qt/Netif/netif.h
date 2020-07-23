#ifndef NETIF_H
#define NETIF_H

#include <QObject>
#include <QString>
#include <QNetworkInterface>
#include <QHostInfo>

class Netif : public QObject
{

    Q_OBJECT

public:
    Netif(QObject *parent = 0) : QObject(parent) { }
    Q_INVOKABLE QString getHostname();
    Q_INVOKABLE QString getIPAddress(QString interface_name);
    Q_INVOKABLE QString getNetmask(QString interface_name);
    Q_INVOKABLE QString getBroadcast(QString interface_name);
    Q_INVOKABLE bool getIFStatus(QString interface_name);

};

#endif // NETIF_H
