package com.rte_france.argus.embedded_powsybl;

import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CTypeConversion;
import java.nio.file.Files;
import java.nio.file.Path;
import java.io.IOException;

public final class IIdmManager {

    @CEntryPoint(name = "readXiidmFile")
    public static CCharPointer readXiidmFile(IsolateThread thread, CCharPointer filePath) {
        String path = CTypeConversion.toJavaString(filePath);
        try {
            if (!path.toLowerCase().endsWith(".xiidm")) {
                return CTypeConversion.toCString("Error: File must have .xiidm extension").get();
            }

            String content = Files.readString(Path.of(path));
            return CTypeConversion.toCString(content).get();
        } catch (IOException e) {
            return CTypeConversion.toCString("Error reading file: " + e.getMessage()).get();
        }
    }
}