package it.ferrarispancaldo.adtools.ldap.interfaces;

/**
 * {@code LdapConnectionManager} interface defines an interface for classes that should
 * manage LDAP connections.
 */
public interface LdapConnectionManager {

    /**
     * Connects to the LDAP server
     */
    void connect();

    /**
     * Disconnects from the LDAP server
     */
    void disconnect();

    /**
     * Checks the LDAP connection status
     * @return true if connected, false otherwise
     */
    boolean isConnected();

}
