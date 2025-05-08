import sys

def print_usage():
    print("Usage: python mifify_css.py <css_dir> <target_dir>")


def main(argv: list[str], argc: int):
    import os

    argv= argv[1::]
    argc -=1

    print(argv, argc)

    if argc != 2:
        print_usage()
        sys.exit(1)

    css_dir = os.path.join(os.getcwd(), argv[0])
    if not os.path.exists(css_dir):
        print("Given css path does not exists")
        print_usage()
        sys.exit(1)

    target_dir = os.path.join(os.getcwd(), argv[1])
    if not os.path.exists(target_dir):
        print("Given target path does not exists")
        print_usage()
        sys.exit(1)

    files = []
    for _,_,f in os.walk(css_dir):
        files = f

    css_content = ""
    for file in files:
        if not file.endswith(".css"):
            print(f"Skipping {file}")
            continue

        with open(f"{css_dir}/{file}", "r", encoding="UTF-8") as f:
            css_content += f.read().strip().replace(" ", "").replace("\n", "")

    with open(f"{target_dir}/style.min.css", "w", encoding="UTF-8") as f:
        f.write(css_content)


if __name__ =="__main__":
    main(sys.argv, len(sys.argv))
