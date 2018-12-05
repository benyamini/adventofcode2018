import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
        ArrayList<String> ids = readFileByLine("data/input");

        int twos = 0;
        int threes = 0;

        for (String id : ids) {
            for (int i = 0; i < id.length(); i++) {
                String sortedID = sortString(id);
                String groupedED = groupString(sortedID);
            }
        }
    }

    public ArrayList<String> readFileByLine(String fileName) {
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

    public String sortString(String inputString) {
        // convert input string to char array
        char tempArray[] = inputString.toCharArray();

        // sort tempArray
        Arrays.sort(tempArray);

        // return new sorted string
        return new String(tempArray);
    }

    public ArrayList<ArrayList<Character>> groupString(String inputString) {
        ArrayList<String> stringList = new ArrayList<String>();
        int listIndex = 0;

        for (int i = 0; i < inputString.length(); i++) {
            ArrayList<Character> tempArray = new ArrayList<Character>();
            if (inputString.charAt(i) == inputString.charAt(i)) {

            }
        }
        // bbaassc
        return new ArrayList<ArrayList<Character>>();
    }
}
