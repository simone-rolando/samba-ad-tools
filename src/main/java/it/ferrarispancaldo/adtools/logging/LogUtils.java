package it.ferrarispancaldo.adtools.logging;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

/**
 * The {@code LogUtils} class manages retrieval of the application logger configuration
 * and instance.
 */
public class LogUtils {
    /**
     * Returns the instance of the logger for this application.
     * @param klass class to log from
     * @return instance of the application logger as {@code Logger}
     */
    public static Logger getLogger(Class<?> klass) {
        return LogManager.getLogger(klass);
    }
}
