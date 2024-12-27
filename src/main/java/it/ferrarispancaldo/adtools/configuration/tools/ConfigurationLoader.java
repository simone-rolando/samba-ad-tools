package it.ferrarispancaldo.adtools.configuration.tools;

import it.ferrarispancaldo.adtools.configuration.types.LocalConfiguration;
import it.ferrarispancaldo.adtools.configuration.types.DomainConfiguration;

import org.yaml.snakeyaml.Yaml;

import java.io.FileInputStream;
import java.io.IOException;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class ConfigurationLoader {
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

    private ConfigurationLoader() {

    }

    private ConfigurationLoader(String localConfigFile, String domainConfigFile) {

    }

    private void loadConfig(String localConfigFile, String domainConfigFile) {
        Yaml localConfig = new Yaml();
        Yaml domainConfig = new Yaml();

        try (FileInputStream localStream = new FileInputStream(localConfigFile)) {
            localConfiguration = localConfig.loadAs(localStream, LocalConfiguration.class);
        } catch (IOException e) {
            e.printStackTrace();
        }

        try (FileInputStream domainStream = new FileInputStream(domainConfigFile)) {
            domainConfiguration = domainConfig.loadAs(domainStream, DomainConfiguration.class);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
