from random import randint

def main():
    N = randint(0, 100)
    guess = -1 # we set -1 here so that it never will collide with randint()
    while guess != N:
        guess = int(input("Enter Guess: "))
        if guess < N:
            print("Guess is Lower than N.")
        elif guess > N:
            print("Guess is Higher than N.")
    print("You have successfully guessed the number")
    
if __name__ == "__main__":
    main()