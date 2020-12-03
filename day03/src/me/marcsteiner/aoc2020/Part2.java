package me.marcsteiner.aoc2020;

import java.io.File;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.List;

public class Part2 {

    public static void main(String[] args) throws IOException {
        List<String> list = Files.readAllLines(new File("input.txt").toPath(), StandardCharsets.UTF_8);

        BigInteger a = new BigInteger(String.valueOf(countTrees(list, 1, 1)));
        BigInteger b = new BigInteger(String.valueOf(countTrees(list, 3, 1)));
        BigInteger c = new BigInteger(String.valueOf(countTrees(list, 5, 1)));
        BigInteger d = new BigInteger(String.valueOf(countTrees(list, 7, 1)));
        BigInteger e = new BigInteger(String.valueOf(countTrees(list, 1, 2)));

        // This results in a long overflow which is why we need to use BigInteger here
        // Lost like a good 20 minutes here because I didn't know this before lol
        BigInteger result = a.multiply(b).multiply(c).multiply(d).multiply(e);

        System.out.println("Trees: " + result);
    }

    private static int countTrees(List<String> lines, int xFactor, int yFactor) {
        int x = 0;
        int y = 0;
        int trees = 0;

        do {
            if (x >= 31) {
                x -= 31;
            }

            String s = lines.get(y);

            char c = s.charAt(x);

            if (c == '#') {
                trees++;
            }

            x += xFactor;
            y += yFactor;
        } while (y < lines.size());

        return trees;
    }

}
