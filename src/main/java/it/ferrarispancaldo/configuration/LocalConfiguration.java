package it.ferrarispancaldo.configuration;

/**
 * The {@code LocalConfiguration} class represents the configuration for a local domain
 * controller configuration.
 *
 * <p>This class is used to hold configuration values deserialized from a configuration
 * file located somewhere on the system.</p>
 */
public class LocalConfiguration {
    /**
     * Local path to users home directories pool
     */
    @SuppressWarnings("unused")
    private String usersHomeDirsPath;

    /**
     * Local path to groups shared pools
     */
    @SuppressWarnings("unused")
    private String groupsPoolPath;

    /**
     * Gets the path to the users' home directories.
     *
     * @return the path to the users' home directories
     */
    public String getUsersHomeDirsPath() {
        return usersHomeDirsPath;
    }

    /**
     * Gets the path to the groups pool.
     *
     * @return the path to the groups pool
     */
    public String getGroupsPoolPath() {
        return groupsPoolPath;
    }
}
