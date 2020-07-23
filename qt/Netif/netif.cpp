#include "netif.h"

QString Netif::getHostname() {
    return QHostInfo::localHostName();
}

QString Netif::getIPAddress(QString interface_name) {

    QString ip_addr;

    QNetworkInterface interface = QNetworkInterface::interfaceFromName(interface_name);

    QList<QNetworkAddressEntry> netInfo = interface.addressEntries();

    if(netInfo.count() > 0) {
        ip_addr = netInfo[0].ip().toString();
    }

    return ip_addr;

}

QString Netif::getNetmask(QString interface_name) {

    QString netmask;

    QNetworkInterface interface = QNetworkInterface::interfaceFromName(interface_name);

    QList<QNetworkAddressEntry> netInfo = interface.addressEntries();

    if(netInfo.count() > 0) {
        netmask = netInfo[0].netmask().toString();
    }

    return netmask;

}

QString Netif::getBroadcast(QString interface_name) {

    QString broadcast_addr;

    QNetworkInterface interface = QNetworkInterface::interfaceFromName(interface_name);

    QList<QNetworkAddressEntry> netInfo = interface.addressEntries();

    if(netInfo.count() > 0) {
        broadcast_addr = netInfo[0].broadcast().toString();
    }

    return broadcast_addr;

}

bool Netif::getIFStatus(QString interface_name) {

    QNetworkInterface interface = QNetworkInterface::interfaceFromName(interface_name);
    if(interface.IsUp) {
        return true;
    }
    else {
        return false;
    }

}
