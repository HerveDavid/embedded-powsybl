package com.rte_france.argus.embedded_powsybl;

import com.powsybl.commons.PowsyblException;
import com.powsybl.iidm.network.Network;
import com.powsybl.loadflow.LoadFlow;
import com.powsybl.nad.NetworkAreaDiagram;
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

    @CEntryPoint(name = "readNetworkFile")
    public static CCharPointer readAndGenerateSvg(IsolateThread thread, CCharPointer filePath) {
        var path = CTypeConversion.toJavaString(filePath);

        try {
            // Get the file name without extension for SVG output
            var fileName = Path.of(path).getFileName().toString();
            var svgFileName = fileName.substring(0, fileName.lastIndexOf('.')) + ".svg";

            // Read the network using PowSyBl
            var network = Network.read(path);
            LoadFlow.run(network);
            NetworkAreaDiagram.draw(network, Path.of(svgFileName));

            return CTypeConversion.toCString("Successfully generated " + svgFileName).get();

        } catch (PowsyblException e) {
            return CTypeConversion.toCString("Error processing network: " + e.getMessage()).get();
        } catch (Exception e) {
            return CTypeConversion.toCString("Unexpected error: " + e.getMessage()).get();
        }
    }


}