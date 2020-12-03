package me.marcsteiner.aoc2020;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.List;

public class Part1 {

    public static void main(String[] args) throws IOException {
        List<String> list = Files.readAllLines(new File("input.txt").toPath(), StandardCharsets.UTF_8);
        int x = 0;
        int y = 0;
        int trees = 0;

        do {
            if (x >= 31) {
                x -= 31;
            }

            String s = list.get(y);

            char c = s.charAt(x);

            char[] c1 = s.toCharArray();

            if (c == '#') {
                trees++;
                c1[x] = 'X';
            } else {
                c1[x] = 'O';
            }

            System.out.println(String.valueOf(c1));

            x += 3;
            y++;
        } while (y < list.size());

        System.out.println("Trees: " + trees);
    }

}
