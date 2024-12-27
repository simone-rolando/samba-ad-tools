package it.ferrarispancaldo.adtools.configuration.tools;

import it.ferrarispancaldo.adtools.configuration.types.LocalConfiguration;
import it.ferrarispancaldo.adtools.configuration.types.DomainConfiguration;
import it.ferrarispancaldo.adtools.logging.LogUtils;

import org.apache.logging.log4j.Logger;
import org.yaml.snakeyaml.Yaml;

import java.io.FileInputStream;
import java.io.IOException;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

/**
 * {@code ConfigurationLoader} loads configuration from the system configuration
 * YAML files.
 *
 * <p>It is possible to load configurations from the <strong>default path</strong>
 * or from a <strong>custom path</strong> passed as parameters.</p>
 */
public class ConfigurationLoader {
    /**
     * System logger instance
     */
    private static final Logger logger = LogUtils.getLogger(ConfigurationLoader.class);

    /**
     * Domain configuration default file
     */
    public static final String DOMAIN_CONFIG_FILE = "/etc/adtools/domain.yaml";

    /**
     * Local configuration default file
     */
    public static final String LOCAL_CONFIG_FILE = "/etc/adtools/local.yaml";

    /**
     * Singleton static instance of this class
     */
    private static ConfigurationLoader instance;

    /**
     * Global concurrency lock
     */
    private static final Lock lock = new ReentrantLock();

    /**
     * Local configuration data
     */
    private LocalConfiguration localConfiguration;

    /**
     * Domain configuration data
     */
    private DomainConfiguration domainConfiguration;

    /**
     * Construct a new {@code ConfigurationLoader}
     * @param localConfigFile path to local configuration file
     * @param domainConfigFile path to domain configuration file
     */
    private ConfigurationLoader(String localConfigFile, String domainConfigFile) {
        loadConfig(localConfigFile, domainConfigFile);
    }

    /**
     * Gets the instance of the {@code ConfigurationLoader}
     * @return instance of the {@code ConfigurationLoader}
     */
    public static ConfigurationLoader getInstance() {
        return getInstance(LOCAL_CONFIG_FILE, DOMAIN_CONFIG_FILE);
    }

    /**
     * Gets the instance of the {@code ConfigurationLoader} from custom config files paths
     * @param localConfigFile custom local configuration file path
     * @param domainConfigFile custom domain configuration file path
     * @return instace of the {@code ConfigurationLoader}
     */
    public static ConfigurationLoader getInstance(String localConfigFile, String domainConfigFile) {
        if (instance == null) {
            instance = new ConfigurationLoader(localConfigFile, domainConfigFile);
        }

        return instance;
    }

    /**
     * Gets the {@code LocalConfiguration} data from the loader
     * @return {@code LocalConfiguration} data
     */
    public LocalConfiguration getLocalConfiguration() {
        return localConfiguration;
    }

    /**
     * Gets the {@code DomainConfiguration} data from the loader
     * @return {@code DomainConfiguration} data
     */
    public DomainConfiguration getDomainConfiguration() {
        return domainConfiguration;
    }

    /**
     * Loads configuration from the provided config files
     * @param localConfigFile path to local configuration file
     * @param domainConfigFile path to domain configuration file
     */
    private void loadConfig(String localConfigFile, String domainConfigFile) {
        Yaml localConfig = new Yaml();
        Yaml domainConfig = new Yaml();

        try (FileInputStream localStream = new FileInputStream(localConfigFile)) {
            localConfiguration = localConfig.loadAs(localStream, LocalConfiguration.class);
        } catch (IOException e) {
            logger.fatal("Error occured while loading local configuration file: {}", localConfigFile, e);
        }

        try (FileInputStream domainStream = new FileInputStream(domainConfigFile)) {
            domainConfiguration = domainConfig.loadAs(domainStream, DomainConfiguration.class);
        } catch (IOException e) {
            logger.fatal("Error occured while loading domain configuration file: {}", domainConfigFile, e);
        }
    }
}
