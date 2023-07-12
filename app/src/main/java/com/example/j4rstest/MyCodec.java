package com.example.j4rstest;

import com.squareup.moshi.Json;
import com.squareup.moshi.JsonClass;
import com.squareup.moshi.Moshi;
import com.squareup.moshi.Types;

import org.astonbitecode.j4rs.api.services.json.Codec;
import org.astonbitecode.j4rs.api.services.json.exceptions.JsonCodecException;

import java.io.IOException;
import java.lang.reflect.Type;
import java.util.List;
import java.util.stream.IntStream;

public class MyCodec implements Codec {
    @JsonClass(generateAdapter =  true)
    public static class RustVariant {
        @Json(name = "class_name")
        private String name;

        private String json;

        private boolean serialized;

        public String getName() {
            return name;
        }

        public void setName(String name) {
            this.name = name;
        }

        public String getJson() {
            return json;
        }

        public void setJson(String json) {
            this.json = json;
        }

        public boolean isSerialized() {
            return serialized;
        }

        public void setSerialized(boolean serialized) {
            this.serialized = serialized;
        }
    }
    @JsonClass(generateAdapter =  true)
    public static class ElementWrapper {
        @Json(name = "Rust") // Right now we only care objects passed from Rust
        private RustVariant variant;

        public RustVariant getVariant() {
            return variant;
        }

        public void setVariant(RustVariant variant) {
            this.variant = variant;
        }
    }

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
        try {
            Type type = Types.newParameterizedType(List.class, ElementWrapper.class);
            List<ElementWrapper> variants = moshi.<List<ElementWrapper>>adapter(type).fromJson(s);
            return IntStream.range(0, variants.size())
                    .mapToObj((index) -> {
                        RustVariant variant = variants.get(index).getVariant();
                        try {
                            return moshi.adapter(Class.forName(variant.name)).fromJson(variant.json);
                        } catch (RuntimeException | ClassNotFoundException | IOException e) {
                            throw new JsonCodecException("Fail to parse #" + index + " argument, type=" + variant.name, e);
                        }
                    }).toArray();
        } catch (RuntimeException | IOException e) {
            throw new JsonCodecException("Decode invocation arguments error", e);
        }
    }
}
