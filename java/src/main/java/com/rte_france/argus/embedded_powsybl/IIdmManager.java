package com.rte_france.argus.embedded_powsybl;

import com.powsybl.commons.PowsyblException;
import com.powsybl.iidm.network.Network;
import com.powsybl.nad.NetworkAreaDiagram;
import com.powsybl.nad.svg.SvgParameters;
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

        // Get the file name without extension for SVG output
        var fileName = Path.of(path).getFileName().toString();
        var svgFileName = fileName.substring(0, fileName.lastIndexOf('.')) + ".svg";

        // Read the network using PowSyBl
        var network = Network.read(path);

        // Generate the network diagram
        var diagram = NetworkAreaDiagram.drawToString(network, new SvgParameters());

        System.out.println(diagram);

        return CTypeConversion.toCString(diagram).get();

    }


}