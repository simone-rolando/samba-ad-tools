package it.ferrarispancaldo.configuration;

/**
 * The {@code DomainConfiguration} class represents the configuration for a domain setup.
 * It contains properties related with the configuration of the Active Directory LDAP
 * connection and some basic information related with user creation and management.
 *
 * <p>This class is used to hold configuration values deserialized from a configuration
 * file located somewhere on the system.</p>
 */
public class DomainConfiguration {
    /**
     * Fully qualified domain name of the AD Realm (LDAP search base)
     */
    private String fqdn;

    /**
     * NT Domain name used for NETBIOS backward compatibility
     */
    private String ntDomain;

    /**
     * Domain Controller fully qualified domain name
     */
    private String dcFqdn;

    /**
     * Domain Controller NT NETBIOS name
     */
    private String dcNtName;

    /**
     * NT UNC path to the users share
     */
    private String ntUsersShare;

    /**
     * NT UNC path to the profiles share (or path in a share)
     */
    private String ntProfilesShare;

    /**
     * Gets the Fully Qualified Domain Name (FQDN) of the domain.
     *
     * @return the FQDN of the domain
     */
    public String getFqdn() {
        return fqdn;
    }

    /**
     * Gets the path (or share) for NT user profiles.
     *
     * @return the path (or share) for NT user profiles
     */
    public String getNtProfilesShare() {
        return ntProfilesShare;
    }

    /**
     * Gets the network share for NT users.
     *
     * @return the network share for NT users
     */
    public String getNtUsersShare() {
        return ntUsersShare;
    }

    /**
     * Gets the NT name of the domain controller.
     *
     * @return the NT name of the domain controller
     */
    public String getDcNtName() {
        return dcNtName;
    }

    /**
     * Gets the Fully Qualified Domain Name (FQDN) of the domain controller.
     *
     * @return the FQDN of the domain controller
     */
    public String getDcFqdn() {
        return dcFqdn;
    }

    /**
     * Gets the NT domain name.
     *
     * @return the NT domain name
     */
    public String getNtDomain() {
        return ntDomain;
    }
}
