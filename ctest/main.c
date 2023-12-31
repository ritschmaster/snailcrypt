#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>

#include "snailcrypt.h"

static char *
cipher = NULL;

static char *
plaintext = NULL;

static char *
hint = NULL;

static char * 
filename = NULL;

static int
encrypted_callback(const char *cipher_new);

static int
decrypted_callback(const char *plaintext_new, const char *hint_new, const char *filename_new);

int
encrypted_callback(const char *cipher_new)
{
	/**************************************************************************
	 * Free "cipher" if set
	 */
	if (cipher)
		free(cipher);

	/**************************************************************************
	 * Get the size of "cipher_new"
	 */
	int cipherlen = strlen(cipher_new) + 1;

	/**************************************************************************
	 * Copy "cipher_new" to "cipher"
	 */
	cipher = malloc(sizeof(char) * cipherlen);
	memcpy(cipher, cipher_new, cipherlen);

	return 0;
}

int
decrypted_callback(const char *plaintext_new, const char *hint_new, const char *filename_new)
{
	/**************************************************************************
	 * Free "plaintext" if set
	 */
	if (plaintext)
		free(plaintext);

	/**************************************************************************
	 * Get the size of "plaintext_new"
	 */
	int plaintextlen = strlen(plaintext_new) + 1;

	/**************************************************************************
	 * Copy "plaintext_new" to "plaintext"
	 */
	plaintext = malloc(sizeof(char) * plaintextlen);
	memcpy(plaintext, plaintext_new, plaintextlen);

	/**************************************************************************
	 * Free "hint" if set
	 */
	if (hint)
		free(hint);

	/**************************************************************************
	 * Get the size of "hint_new"
	 */
	int hintlen = strlen(hint_new) + 1;

	/**************************************************************************
	 * Copy "hint_new" to "hint"
	 */
	hint = malloc(sizeof(char) * hintlen);
	memcpy(hint, hint_new, hintlen);

	/**************************************************************************
	 * Free "filename" if set
	 */
	if (filename)
		free(filename);

	/**************************************************************************
	 * Get the size of "filename_new"
	 */
	int filenamelen = strlen(filename_new) + 1;

	/**************************************************************************
	 * Copy "filename_new" to "filename"
	 */
	filename = malloc(sizeof(char) * filenamelen);
	memcpy(filename, filename_new, filenamelen);

	return 0;
}

static void
test_encrypt_vguess_small_str(void)
{
	char *plaintext_orig = "hello world";
	char *hint_orig = "";
	char *filename_orig = "";

	int encryption_result = snailcrypt_ez_encrypt(plaintext_orig, "2022-11-19T17:00:00+0100", hint_orig, filename_orig, encrypted_callback);
	assert(encryption_result == 0);
	assert(cipher);

	int decryption_result = snailcrypt_ez_decrypt(cipher, &decrypted_callback);
	assert(decryption_result == 0);
	assert(plaintext);
	assert(hint_orig);
	assert(filename_orig);

	assert(strcmp(plaintext_orig, plaintext) == 0);
	assert(strcmp(hint_orig, hint) == 0);
	assert(strcmp(filename_orig, filename) == 0);
}

static void
test_encrypt_vguess_large_str(void)
{
	char *plaintext_orig = "Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo,"
"quis tempor ligula erat quis odio.  Nunc porta vulputate tellus."
"Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere."
"Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada"
"massa, quis varius mi purus non odio.  Pellentesque condimentum,"
"magna ut suscipit hendrerit, ipsum augue ornare nulla, non"
"luctus diam neque sit amet urna.  Curabitur vulputate vestibulum"
"lorem.  Fusce sagittis, libero non molestie mollis, magna orci"
"ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula"
"quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.";
	char *hint_orig = "";
	char *filename_orig = "";

	int encryption_result = snailcrypt_ez_encrypt(plaintext_orig, "2022-11-19T17:00:00+0100", hint_orig, filename_orig, encrypted_callback);
	assert(encryption_result == 0);
	assert(cipher);

	int decryption_result = snailcrypt_ez_decrypt(cipher, &decrypted_callback);
	assert(decryption_result == 0);
	assert(plaintext);
	assert(hint_orig);
	assert(filename_orig);

	assert(strcmp(plaintext_orig, plaintext) == 0);
	assert(strcmp(hint_orig, hint) == 0);
	assert(strcmp(filename_orig, filename) == 0);
}

int main(void) {
	test_encrypt_vguess_small_str();
	test_encrypt_vguess_large_str();

	/**************************************************************************
	 * Free "cipher" if set
	 */
	if (cipher)
		free(cipher);

	/**************************************************************************
	 * Free "plaintext" if set
	 */
	if (plaintext)
		free(plaintext);

	return 0;
}
