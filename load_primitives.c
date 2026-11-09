static int cmp_words(const void *a, const void *b){
    return strcmp(*(const char * const *)a, *(const char * const *)b);
}

void load_primitives(void){
    DIR *dir = opendir("words");
    struct dirent *entry;
    size_t total = 0;
    size_t count = 0;

    if (dir == NULL) {
        perror("opendir");
        return;
    }

    // Prima passata: conta file e caratteri
    while ((entry = readdir(dir)) != NULL) {
        const char *name = entry->d_name;
        size_t len = strlen(name);
        if (len >= 3 && strcmp(name + len - 2, ".c") == 0) {
            total += len-2 ;
            count++;
        }
    }

    if (count == 0) {
        closedir(dir);
        return;
    }

    // Alloca array di puntatori e stringhe
    char **tmp = calloc(count , sizeof(char *));
    char *data = calloc(total ,sizeof(data[0]) );
    used= calloc( total, sizeof(used[0]) );

    if (tmp == NULL || data == NULL) {
        free(tmp);
        free(data);
        closedir(dir);
        return;
    }

    // Seconda passata
    rewinddir(dir);
    size_t i = 0;
    size_t offset = 0;

    while ((entry = readdir(dir)) != NULL && i < count) {
        const char *name = entry->d_name;
        size_t len = strlen(name);
        if (len >= 3 && strcmp(name + len - 2, ".c") == 0) {
            tmp[i] = data + offset;
            memcpy(tmp[i], name, len -2);
            offset += len + 1;
            i++;
        }
    }
    closedir(dir);

    // readdir() returns entries in filesystem order, which varies between
    // machines: sort so the generated files are always identical
    qsort(tmp, i, sizeof(tmp[0]), cmp_words);

    words = tmp;
    nwords = i;
}
