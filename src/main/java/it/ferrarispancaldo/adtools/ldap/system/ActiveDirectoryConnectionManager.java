package it.ferrarispancaldo.adtools.ldap.system;

import it.ferrarispancaldo.adtools.logging.LogUtils;
import it.ferrarispancaldo.adtools.ldap.interfaces.LdapConnectionManager;

import org.apache.directory.api.ldap.model.schema.registries.SchemaLoader;
import org.apache.directory.api.ldap.schema.manager.impl.DefaultSchemaManager;
import org.apache.directory.ldap.client.api.DefaultSchemaLoader;
import org.apache.logging.log4j.Logger;

import org.apache.directory.api.ldap.model.name.Dn;
import org.apache.directory.api.ldap.model.schema.SchemaManager;
import org.apache.directory.ldap.client.api.LdapConnection;
import org.apache.directory.ldap.client.api.LdapNetworkConnection;
import org.apache.directory.api.ldap.model.message.BindRequestImpl;

/**
 * Class {@code ActiveDirectoryConnectionManager} implements an Active Directory
 * connection management class using the LdapConnectionManager unified interface.
 *
 * <p><strong>WARNING:</strong> This should only be used to connect to Active Directory domains
 * as standard LDAP schemas differ and the binding process is completely
 * different</p>
 */
public class ActiveDirectoryConnectionManager implements LdapConnectionManager {

    // Class logger
    private final Logger logger = LogUtils.getLogger(this.getClass());

    /**
     * Connection object
     */
    private LdapConnection connection;

    /* LDAP connection settings */
    private final String host;
    private final int port;
    private final String domain;

    /**
     * Creates a new ActiveDirectoryConnectionManager
     * @param host server IP address or FQDN
     * @param port server Active Directory LDAP port
     * @param domain Fully Qualified Domain Name of Active Directory realm
     */
    public ActiveDirectoryConnectionManager(String host, int port, String domain) {
        this.host = host;
        this.port = port;
        this.domain = domain;
    }

    @Override
    public void connect() {
        if (connection != null && connection.isConnected()) return;

        if (connection == null || !connection.isConnected()) {
            connection = new LdapNetworkConnection(host, port);

            try {
                connection.connect();
                logger.debug("Active Directory connection established on {}:{}", host, port);
            } catch (Exception e) {
                throw new RuntimeException("Failed to establish Active Directory connection", e);
            }
        }
    }

    @Override
    public void disconnect() {
        if (connection == null || !connection.isConnected()) return;

        try {
            connection.unBind();
            connection.close();
            logger.debug("Active Directory connection with {}:{} closed successfully", host, port);
        } catch (Exception e) {
            throw new RuntimeException("Failed to close active Active Directory connection", e);
        }
    }

    @Override
    public boolean isConnected() {
        return (connection != null && connection.isConnected());
    }

    // TODO: add implementation of bind()
    public boolean bind(String username, String password) {
        return false;
    }
}
