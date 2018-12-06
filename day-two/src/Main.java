import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class Main {

    public static void main(String[] args) {
        ArrayList<String> ids = readFileByLine("data/input");

        HashMap<String, HashMap<Character, Integer>> frequencyMap = new HashMap<String, HashMap<Character, Integer>>();

        int twos = 0;
        int threes = 0;

        for (int s = 0; s < ids.size(); s++) {
            String sortedId = ids.get(s);
            HashMap<Character, Integer> idMap = new HashMap<Character, Integer>();

            for (int i = 0; i < sortedId.length(); i++) {
                Integer count = idMap.get(sortedId.charAt(i));
                if (count == null) {
                    idMap.put(sortedId.charAt(i), 1);
                } else {
                    idMap.put(sortedId.charAt(i), count + 1);
                }
            }
            frequencyMap.put(sortedId, idMap);
        }

        for (Map.Entry<String, HashMap<Character, Integer>> id : frequencyMap.entrySet()) {
            HashMap<Character, Integer> counts = id.getValue();

            int addToTwo = 0;
            int addToThree = 0;

            for (Map.Entry<Character, Integer> count : counts.entrySet()) {
                if (count.getValue() == 2 && addToTwo == 0) {
                    addToTwo = 1;
                }
                if (count.getValue() == 3 && addToThree == 0) {
                    addToThree = 1;
                }
            }
            twos += addToTwo;
            threes += addToThree;
        }
        System.out.println(frequencyMap);
        System.out.println(twos);
        System.out.println(threes);
        System.out.println(twos * threes);
    }

    public static ArrayList<String> readFileByLine(String fileName) {
        ArrayList<String> ids = new ArrayList<String>();
        try {
            File file = new File(fileName);
            Scanner scanner = new Scanner(file);
            while (scanner.hasNext()) {
                ids.add(scanner.next());
            }
            scanner.close();
            return ids;
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
        return ids;
    }
}
