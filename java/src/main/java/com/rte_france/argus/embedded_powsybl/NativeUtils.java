package com.rte_france.argus.embedded_powsybl;

import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CTypeConversion;

public class NativeUtils {

    @CEntryPoint(name = "Java_com_rte_france_argus_embedded_powsybl_NativeUtils_factorial")
    public static long factorial(IsolateThread thread, long n) { // Ajout du paramètre IsolateThread
        if (n < 0) {
            throw new IllegalArgumentException("Le nombre doit être positif");
        }
        if (n == 0 || n == 1) {
            return 1;
        }
        return n * factorial(thread, n - 1); // N'oubliez pas de passer le thread lors de l'appel récursif
    }

    @CEntryPoint(name = "Java_com_rte_france_argus_embedded_powsybl_NativeUtils_concatenate")
    public static CCharPointer concatenate(IsolateThread thread, CCharPointer str1, CCharPointer str2) {
        String javaStr1 = CTypeConversion.toJavaString(str1);
        String javaStr2 = CTypeConversion.toJavaString(str2);
        String result = javaStr1 + javaStr2;
        return CTypeConversion.toCString(result).get();
    }
}