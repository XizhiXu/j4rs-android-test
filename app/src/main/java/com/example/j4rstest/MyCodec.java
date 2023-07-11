package com.example.j4rstest;

import com.squareup.moshi.Moshi;

import org.astonbitecode.j4rs.api.services.json.Codec;
import org.astonbitecode.j4rs.api.services.json.exceptions.JsonCodecException;

import java.io.IOException;

public class MyCodec implements Codec {
    private final Moshi moshi = new Moshi.Builder().build();

    @Override
    public <T> T decode(String s, String className) throws JsonCodecException {
        try {
            return moshi.<T>adapter(Class.forName(className)).fromJson(s);
        } catch (RuntimeException | ClassNotFoundException | IOException e) {
            throw new JsonCodecException("Decode j4rs object error, type=" + className, e);
        }
    }

    @Override
    public String encode(Object o) throws JsonCodecException {
        try {
            return null;
        } catch (RuntimeException e) {
            throw new JsonCodecException("Encode j4rs object error", e);
        }
    }

    @Override
    public Object[] decodeArrayContents(String s) throws JsonCodecException {
        return new Object[0];
    }
}
